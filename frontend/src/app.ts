import './logger';
import { v4 as uuidv4 } from 'uuid';
import { syncInitialState, syncEvents } from './sync';
import { addEvent, getAllPosts, putPost, Post } from './idb';

const postsContainer = document.getElementById('posts-container') as HTMLDivElement;
const messageContainer = document.getElementById('message-container') as HTMLDivElement;
const searchBar = document.getElementById('search-bar') as HTMLInputElement;
let user_token: string;
let posts: Post[] = [];
let filteredPosts: Post[] = [];

// --- Scoring ---

function totalScore(post: Post): number {
  return post.base_score + post.user_score;
}

// --- UI Rendering ---

function createDynamicPlaceholder(hash: string): string {
  const hue1 = parseInt(hash.substring(0, 3), 16) % 360;
  const hue2 = parseInt(hash.substring(3, 6), 16) % 360;

  const c1Hue = parseInt(hash.substring(6, 9), 16) % 360;
  const c1X = parseInt(hash.substring(9, 11), 16) % 100;
  const c1Y = parseInt(hash.substring(11, 13), 16) % 100;
  const c1R = 15 + (parseInt(hash.substring(13, 15), 16) % 10);

  const c2Hue = parseInt(hash.substring(15, 18), 16) % 360;
  const c2X = parseInt(hash.substring(18, 20), 16) % 100;
  const c2Y = parseInt(hash.substring(20, 22), 16) % 100;
  const c2R = 15 + (parseInt(hash.substring(22, 24), 16) % 15);
  
  const c3Hue = parseInt(hash.substring(24, 27), 16) % 360;
  const c3X = parseInt(hash.substring(27, 29), 16) % 100;
  const c3Y = parseInt(hash.substring(29, 31), 16) % 100;
  const c3R = 15 + (parseInt(hash.substring(31, 33), 16) % 10);

  const svgString = `
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 100 100" width="100" height="100">
      <defs>
        <linearGradient id="grad-${hash}" x1="0%" y1="0%" x2="100%" y2="100%">
          <stop offset="0%" style="stop-color:hsl(${hue1}, 70%, 60%)" />
          <stop offset="100%" style="stop-color:hsl(${hue2}, 80%, 75%)" />
        </linearGradient>
        <filter id="shadow-${hash}" x="-20%" y="-20%" width="140%" height="140%">
          <feGaussianBlur in="SourceAlpha" stdDeviation="2"/>
          <feOffset dx="2" dy="2" result="offsetblur"/>
          <feComponentTransfer>
            <feFuncA type="linear" slope="0.3"/>
          </feComponentTransfer>
          <feMerge> 
            <feMergeNode/>
            <feMergeNode in="SourceGraphic"/> 
          </feMerge>
        </filter>
      </defs>
      <rect x="0" y="0" width="100" height="100" fill="url(#grad-${hash})"/>
      <circle cx="${c1X}" cy="${c1Y}" r="${c1R}" fill="hsl(${c1Hue}, 70%, 80%)" opacity="0.4"/>
      <circle cx="${c2X}" cy="${c2Y}" r="${c2R}" fill="hsl(${c2Hue}, 70%, 80%)" opacity="0.5"/>
      <circle cx="${c3X}" cy="${c3Y}" r="${c3R}" fill="hsl(${c3Hue}, 70%, 80%)" opacity="0.3"/>
      <text x="50%" y="55%" dominant-baseline="middle" text-anchor="middle" font-family="sans-serif" font-size="40" font-weight="bold" fill="white" filter="url(#shadow-${hash})">
        DT
      </text>
    </svg>
  `;
  return `data:image/svg+xml;base64,${btoa(svgString)}`;
}

function renderPosts() {
  filteredPosts.sort((a, b) => totalScore(b) - totalScore(a));
  
  postsContainer.innerHTML = '';
  filteredPosts.forEach(post => {
    const postElement = document.createElement('div');
    postElement.dataset.hash = post.post_id;

    let contentHtml;
    try {
      const musicData = JSON.parse(post.content);
      if (musicData && musicData.type === 'music') {
        const imageUrl = musicData.thumbnail_url || createDynamicPlaceholder(post.post_id);
        const imageElement = `<img src="${imageUrl}" alt="Cover Art" width="100" style="object-fit: cover; aspect-ratio: 1/1;">`;
        
        contentHtml = `
          <div style="display: flex; align-items: center; gap: 15px;">
            ${imageElement}
            <div style="flex-grow: 1;">
              <h4>${musicData.title} - ${musicData.artist}</h4>
            </div>
          </div>
        `;
      } else {
        throw new Error("Not a music post");
      }
    } catch {
      contentHtml = `<p>${post.content}</p>`;
    }

    postElement.innerHTML = `
      <div>
        ${contentHtml}
        <input type="range" min="-127" max="128" value="${post.user_score}" class="vote-slider">
      </div>
    `;
    postsContainer.appendChild(postElement);
  });
}

// --- Event Handlers ---

function filterPosts() {
  const searchTerm = searchBar.value.toLowerCase();
  filteredPosts = posts.filter(post => {
    try {
      const musicData = JSON.parse(post.content);
      if (musicData && musicData.type === 'music') {
        return musicData.title.toLowerCase().includes(searchTerm) ||
               musicData.artist.toLowerCase().includes(searchTerm);
      }
    } catch {
      // Not a JSON object, fall through to text search
    }
    return post.content.toLowerCase().includes(searchTerm);
  });
  renderPosts();
}

searchBar.addEventListener('input', filterPosts);

postsContainer.addEventListener('input', (e) => {
  const target = e.target as HTMLInputElement;
  if (target.classList.contains('vote-slider')) {
    const postElement = target.closest('div[data-hash]') as HTMLDivElement;
    const post_id = postElement.dataset.hash!;
    const post = posts.find(p => p.post_id === post_id);
    if (post) {
      post.user_score = parseInt(target.value, 10);
      renderPosts();
    }
  }
});

postsContainer.addEventListener('change', async (e) => {
  const target = e.target as HTMLInputElement;
  if (target.classList.contains('vote-slider')) {
    const postElement = target.closest('div[data-hash]') as HTMLDivElement;
    const post_id = postElement.dataset.hash!;
    const score = parseInt(target.value, 10);
    
    await addEvent({
      client_key: uuidv4(),
      user_token,
      action: 'vote',
      payload: { content_hash: post_id, score },
      created_at: Date.now(),
    });
    syncEvents();
    const post = posts.find(p => p.post_id === post_id);
    if (post) {
      await putPost(post);
    }
  }
});

async function loadPosts() {
  posts = await getAllPosts();
  filterPosts();
}

function initializeUserToken() {
  const regex = /[?&]st=([^&]*)/;
  const match = regex.exec(window.location.search);
  const token = match ? decodeURIComponent(match[1].replace(/\+/g, ' ')) : null;

  if (token) {
    user_token = token;
    localStorage.setItem('user_token', token);
  } else {
    user_token = localStorage.getItem('user_token') || '';
  }

  if (!user_token) {
    messageContainer.textContent = "No user token provided. Please access via a valid URL.";
    messageContainer.style.display = 'block';
  }
}

// --- Initialization ---

(async () => {
  initializeUserToken();
  await loadPosts();
  await syncInitialState();
  await loadPosts();
  setInterval(syncEvents, 5000);
})();
