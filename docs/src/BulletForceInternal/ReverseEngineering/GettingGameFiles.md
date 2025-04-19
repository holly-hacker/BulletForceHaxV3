# Getting copies of game files

There are various files that may be useful while reverse engineering Photon Unity Networking or Bullet Force. This page describes how to get some of them.

## Bullet Force

### Bullet Force WebGL (web build)

You can download the WebGL game files by inspecting network traffic through your browser's developer tools or a HTTP/HTTPS proxy.

The copy of the game hosted on CrazyGames serves the following files which may be of interest:
- https://files.crazygames.com/bullet-force-multiplayer/321/Build/bfweb4.loader.js
- https://files.crazygames.com/bullet-force-multiplayer/321/Build/bfweb4.framework.js.gz
- https://files.crazygames.com/bullet-force-multiplayer/321/Build/bfweb4.data.gz
- https://files.crazygames.com/bullet-force-multiplayer/321/Build/bfweb4.wasm.gz

The location of these files may, and are likely to, change in the future. To ensure you have the latest game files, find the latest URLs through DevTools.

### Bullet Force APK (Android build)

You can get a copy of the Bullet Force APK from [APKMirror](https://www.apkmirror.com/?s=bullet+force). You may find multiple variants even within a version number, but pick the one uploaded last. You can expect this file to be ~500mb in size.

You should end up with an `.apkm` file, which is a "split" APK. You can change the file extension to `.zip` and extract it which should result in a directory structure similar to this:

```
.
├── APKM_installer.url
├── base.apk
├── icon.png
├── info.json
├── META-INF
│   ├── APKMIRRO.RSA
│   ├── APKMIRRO.SF
│   └── MANIFEST.MF
├── split_config.arm64_v8a.apk
├── split_config.armeabi_v7a.apk
└── split_UnityDataAssetPack.apk
```

## Photon

### Photon Unity Networking

You can download a copy of Photon Unity Networking by installing Unity, creating a new project, and installing it from the Asset Store.

You may also be able to find uploaded copies on GitHub.

### Photon Server

Photon uses an on-premise version of the Photon Server. This handles all server-side logic, excluding the Bullet Force-specific extensions/plugins. It can be useful to provide insight into how Photon handles connections on the server side, and could perhaps be used to create a privately hosted server.

The current `v5.x.x.x` Photon Server SDK seems to only be accessible to paying [Photon Circle Members](https://www.photonengine.com/gaming/pricing) which starts at $125/month with a minimum initial term of 3 months.

You may be able to find copies uploaded online, such as [this `v4.0.25.11263` version](https://archive.org/details/photon-server-sdk_v4-0-29-11263) from 2023 which used to be available as a free download on the Photon website. As Bullet Force appears to be using v4 of PUN, this version is likely fairly close to what Bullet Force uses.
