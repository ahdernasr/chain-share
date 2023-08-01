# ChainShare

A blockchain-based decentralised file-sharing system that enables file sharing through a local peer-to-peer network.

### General Information

The program works through a single blockchain instance that holds the files that are uploaded (and can be downloaded) shared by different peers in a peer-to-peer network. Upon instructing the cli to upload a file, a block in the blockchain is mined and added to the blockchain holding the user's chosen file's data. From there, any member of the peer-to-peer network can also add files to the blockchain or they can download any of the files added by peers in the network.

### How to use 

Open two terminal windows on your device, or two terminal windows on two different devices connected to the same network, and start the cli using the following command on each terminal:

```console
cargo run 
```

The following screen should appear, and you should recieve an alert that a peer has been found (if there are 2 or more instances of the terminal window running):

![Screenshot 2023-07-31 at 21 09 35](https://github.com/ahdernasr/blockchain-file-sharing/assets/44983175/f33e3758-e6e5-4242-9641-7c8c6c26c34b)


From then on, use the following command to view how to upload and download files between peers in the network using blockchain and p2p network capabilities:

```console
guide
```

### TODO

There are a few things that need to be added to the project to ensure speed and usability:
- Use 'path-clean' or 'path-abs' libraries to handle paths more robustly
- Extend peer to peer run time and find out what shortens it
- Find out the amount of time it takes for mdns to look for a peer and how to speed it up
