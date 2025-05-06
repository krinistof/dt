# Featues for v0.2
## Simpler interface for better UX
Remove the slider, and make each card slideable left to right for scoring. Animate changing order.

## Song preview
Double tapping on a song card changes to a preview mode, where the voter can listen to the song on their device.

## Toggleable dark and white apperance
With a icon button on the top left, change between dark and light mode.

## Explore songs directory for changes
If a new directory gets added to the music directory, add it to the database.

# Featues for v1.0
## Modularize sources, refactorization
Break `main.rs` into modules with libraries. Prepare conditional compilations targeting either local or serverless deployment.

## Online Song downloader
Download songs and add them to the queue on demand from Spotify, Youtube.
cargo feature: demander
