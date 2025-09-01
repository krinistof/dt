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
        contentHtml = `
          <h4>${musicData.title} - ${musicData.artist}</h4>
          <audio controls preload="none" src="${musicData.url}"></audio>
          <p>Length: ${musicData.length}s</p>
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
