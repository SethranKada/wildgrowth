# WildGrowth Server

## Dev Roadmap

### Features

#### User functions

   1. Users should be able to view, rate, and discuss books that other users have published.
   2. Users should be able to write and edit books that they have published.
   3. Users should be able to comment on and discuss individual chapters.
   4. Users should be able to report errors and suggestions directly to a books authors.
   5. Users should be able to add private notes, comments, and highlights to books and chapters.
   6. Users should be able to create private and public collections of books.
      - Optionally collections can include sequence and other series metadata.
   7. Users should be able to follow authors and books, and get notified when they update or release a new book.
   8. Users should be able to download books or collections to their device.
   9. Users should be able to report a book or author for abuse, impersonation, or other issues to their instance owner. Reports are private.

#### Admin functions

   1. Admins should be able to upload read-only archives of books with custom metadata.
   2. Admins should be able to designate authors as Archived, making it read-only unless the owner requests otherwise.
   3. Admins should be able to designate their Instance as an Archive, disabling unnecessary features to conserve resources.
   4. Admins should be able to read all reports on their instance and address them.
   5. Admins should be able to configure their instance without code or machine-level access.
   6. Admins should have separate permission profiles, where each has access to only what admin tools they are allowed.

### Node Types

   1. Client Node (Mobile / Web):
      - Must connect to a server and remain online to use all functionality
      - May have a cache to allow limited offline viewing.

   2. Peer Node (Linux, Windows, Mac):
      - Uses SSB for communication.
      - Offline first, and only needs to be online to connect to it's home server.
      - should be able to sync with other peers while offline.

   3. Server Node (Docker):
      - Uses ActivityPub for communication.
      - Provides processing and storage functionality to it's users.
      - Expected to remain online at all times.

   4. Bundled CLI Application:
      - Include a command-line interface bundled with Peer and Server Nodes for debugging and testing purposes.
      - The CLI should have features for managing connections, configuring settings, and monitoring network activity.
      - Provide commands for inspecting node status, accessing logs, and performing basic administrative tasks.
