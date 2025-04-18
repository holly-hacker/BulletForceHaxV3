# Introduction

This book holds documentation for [Photon (PUN)][pun], [Bullet Force][bfweb]'s internals and [BulletForceHaxV3][bfhaxv3] (a cheat for Bullet Force).

Bullet Force is a multiplayer first-person shooter game published by [Blayze Games][blayze]. It has been released for [web][bfweb] and mobile (both [Android][bfandroid] and [iOS][bfios]), with a [native PC port][bfsteam] coming to Steam in the near future.

## Why BulletForceHax?

Bullet Force uses [Photon Unity Networking (PUN)][pun] for networking, meaning that most of the networking logic is not implemented by the Bullet Force developers but part of a pre-made package. PUN is also client-authoritive in many aspects, requiring specific hooks/plugins by the game developer to add additional security measures instead of being secure by default.

Additionally, Bullet Force targets Unity's WebGL platform which means tradtional hacking techniques (such as code injection and memory manipulation) are less trivial. It allows for experimenting with different approaches as there is no single way that is clearly better than the rest.

These factors mean that Bullet Force is a fun target to reverse engineer and write tooling for. It is unlike games that cheats are traditionally written for (like Assault Cube or Counter-Strike 2) where the same techniques carry over between games, and to my knowledge had not seen any open source cheats until the original release of [BulletForceHax][bfhaxv1].

[BulletForceHax][bfhaxv1] has since been succeeded by [BulletForceHaxV2][bfhaxv2] and then later by [BulletForceHaxV3][bfhaxv3], which is where my current focus lies. This book will not cover old iterations of BulletForceHax and you should not expect any further development or support for them.

[pun]: https://www.photonengine.com/pun
[blayze]: https://blayzegames.com/
[bfweb]: https://www.crazygames.com/game/bullet-force-multiplayer
[bfandroid]: https://play.google.com/store/apps/details?id=com.blayzegames.iosfps
[bfios]: https://apps.apple.com/us/app/bullet-force/id1009134067
[bfsteam]: https://store.steampowered.com/app/450240/Bullet_Force/
[bfhaxv1]: https://github.com/holly-hacker/BulletForceHax
[bfhaxv2]: https://github.com/holly-hacker/BulletForceHaxV2
[bfhaxv3]: https://github.com/holly-hacker/BulletForceHaxV3
