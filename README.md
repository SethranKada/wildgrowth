# WildGrowth Program Draft

1. User Experience:
   - Provide clear and concise instructions for users to navigate and operate the program.
   - Display helpful error messages or usage hints when users make mistakes or encounter issues.

2. Node Types:
   a. Client Node:
      - Users can connect to a Server Node using a Client Node (e.g., mobile and web applications).
      - Clients should have a simple and intuitive interface for sending and receiving messages or updates.

   b. Peer Node:
      - Peer Nodes operate alongside Server Nodes and utilize peer-to-peer (P2P) connections.
      - Peer Nodes do not need to be online at all times and may have some performance limitations.
      - These nodes are ideal for desktop and laptop applications or single-user servers.
      - Peer Nodes should have basic functionalities similar to Client Nodes.

   c. Server Node:
      - Server Nodes also use P2P connections but primarily rely on Activity Pub for data transfer.
      - Server Nodes are expected to remain online for extended periods.
      - Server Nodes should be capable of forming isolated networks when disconnected from the wider web.
      - Provide robust data storage and synchronization mechanisms for handling large amounts of data.

3. Bundled CLI Application:
   - Include a command-line interface bundled with Peer and Server Nodes for debugging and testing purposes.
   - The CLI should have features for managing connections, configuring settings, and monitoring network activity.
   - Provide commands for inspecting node status, accessing logs, and performing basic administrative tasks.
