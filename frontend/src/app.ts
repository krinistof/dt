import './logger';
import { v4 as uuidv4 } from 'uuid';
import { syncInitialState, syncEvents } from './sync';
import { addEvent, getAllPosts, putPost, Post } from './idb';
// @ts-ignore
import { sha256 } from 'js-sha256';

const postsContainer = document.getElementById('posts-container') as HTMLDivElement;
const postForm = document.getElementById('post-form') as HTMLFormElement;
const postContent = document.getElementById('post-content') as HTMLTextAreaElement;
let user_token: string;
let posts: Post[] = [];

// --- Hashing ---

async function hashContent(content: string): Promise<string> {
  return sha256(content);
}

// --- UI Rendering ---

function renderPosts() {
  posts.sort((a, b) => b.totalScore - a.totalScore);
  
  postsContainer.innerHTML = '';
  posts.forEach(post => {
    const postElement = document.createElement('div');
    postElement.dataset.hash = post.content_hash;
    postElement.innerHTML = `
      <div>
        <p>${post.content}</p>
        <p class="score">
          Score: ${post.totalScore} 
          (Base: ${post.base_score}, User: ${post.user_score})
        </p>
        <input type="range" min="-127" max="128" value="${post.user_score}" data-hash="${post.content_hash}">
      </div>
      <hr>
    `;
    postsContainer.appendChild(postElement);
  });
}

// --- Event Handlers ---

postForm.addEventListener('submit', async (e) => {
  e.preventDefault();
  const content = postContent.value.trim();
  if (!content) return;

  const content_hash = await hashContent(content);

  const newPost = new Post({
    content_hash,
    content,
    base_score: 0,
    user_score: 0,
  });

  posts.push(newPost);
  renderPosts();
  
  await addEvent({
    client_key: uuidv4(),
    user_token,
    action: 'post',
    payload: { content_hash, content },
    created_at: Date.now(),
  });
  syncEvents();
  await putPost(newPost);
  
  postContent.value = '';
});

postsContainer.addEventListener('input', (e) => {
  const target = e.target as HTMLInputElement;
  if (target.type !== 'range') return;

  const content_hash = target.dataset.hash as string;
  const user_score = parseInt(target.value, 10);

  const post = posts.find(p => p.content_hash === content_hash);
  if (!post) return;

  const postElement = postsContainer.querySelector(`[data-hash="${content_hash}"]`);
  if (!postElement) return;

  const scoreElement = postElement.querySelector('.score');
  if (scoreElement) {
    scoreElement.textContent = `Score: ${post.base_score + user_score} (Base: ${post.base_score}, User: ${user_score})`;
  }
});

postsContainer.addEventListener('change', async (e) => {
  const target = e.target as HTMLInputElement;
  if (target.type !== 'range') return;

  const content_hash = target.dataset.hash as string;
  const user_score = parseInt(target.value, 10);

  const postToUpdate = posts.find(p => p.content_hash === content_hash);

  if (postToUpdate) {
    postToUpdate.user_score = user_score;
    renderPosts();
    
    await addEvent({
      client_key: uuidv4(),
      user_token,
      action: 'vote',
      payload: { content_hash, score: user_score },
      created_at: Date.now(),
    });
    syncEvents();
    await putPost(postToUpdate);
  }
});

// --- Initialization ---

async function loadPosts() {
  posts = await getAllPosts();
  renderPosts();
}

function initializeUserToken() {
  const url = window.location.href;
  const match = url.match(/[?&]st=([^&]+)/);
  const tokenFromUrl = match ? match[1] : null;

  if (tokenFromUrl) {
    localStorage.setItem('user_token', tokenFromUrl);
    history.replaceState({}, document.title, window.location.pathname);
    user_token = tokenFromUrl;
  } else {
    user_token = localStorage.getItem('user_token') || '';
  }

  if (!user_token) {
    (postContent as HTMLTextAreaElement).disabled = true;
    (postContent as HTMLTextAreaElement).placeholder = "No user token provided. Please access via a valid URL.";
    (postForm.querySelector('button') as HTMLButtonElement).disabled = true;
  }
}

async function init() {
  window.addEventListener('datachanged', loadPosts);
  initializeUserToken();
  await syncInitialState();
  await loadPosts();
  await syncEvents();

  setInterval(syncEvents, 5000);
}

init();
