# Featues for v0.2
## Simpler interface for better UX
- Stop queue fetching while user interacts.
- Show the currently playing song as inactive on the top of the queue.
- Animate changing order.
- Remove the slider, and make each card slideable left to right for scoring.

## Self initialization
For better deployment experience, if no database exists, create one.

## Song preview
Double tapping on a song card changes to a preview mode, where the voter can listen into the song on their device.

## Toggleable dark and white apperance
With a ic on button on the top left, change between dark and light mode.

# Featues for v1.0
## Modularize sources, refactorization
Break `main.rs` into modules with libraries. Prepare conditional compilations targeting either local or serverless deployment.

## Explore songs directory for changes
If a new directory gets added to the music directory, add it to the database.

## Online Song downloader
Download songs and add them to the queue on demand from Spotify, Youtube.
cargo feature: demander
