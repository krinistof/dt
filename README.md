## Democratic Tier v0.1: Music Voting System

Democratic Tier is a web application that allows users to collaboratively decide the music queue for an event or gathering. Built with Rust (Actix-web) and HTMX, it provides a dynamic and responsive experience without complex client-side JavaScript frameworks. Everything is local, so no internet is required for robustness.

**Core Features:**

*   **Voter Interface:** Users can view a list of available songs and cast weighted votes (using sliders) for their preferred tracks. The song list and scores update dynamically.
*   **Host Control Panel:** A separate interface for the host to manage playback, view all available songs, and optionally autoplay the highest-voted, unplayed songs from the queue.
*   **Real-time Updates:** HTMX is used to refresh the song queue and scores, providing immediate feedback on voting.
*   **Persistent Storage:** Song information, votes, and played status are stored in an SQLite database.

