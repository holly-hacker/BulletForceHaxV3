# Introduction

This book holds documentation for [Photon/PUN][pun], [Bullet Force][bfweb]'s internals and [BulletForceHaxV3][bfhaxv3], a cheat for Bullet Force.

Bullet Force is a multiplayer first-person shooter game published by [Blayze Games][blayze]. It has released for [web][bfweb], [Android][bfandroid] and [iOS][bfios]), with a [native PC port][bfsteam] coming to Steam in the near future.

## Why BulletForceHax?

Bullet Force uses [Photon Unity Networking, or PUN][pun], for networking, meaning that most of the networking logic isn't implemented by the Bullet Force developers but part of a pre-made package. PUN is also client-authoritive in many aspects, requiring specific hooks/plugins by the game developer to add additional security measures instead of being secure by default.

The game's name is BulletForce.

Additionally, Bullet Force targets Unity's WebGL platform which means traditional hacking techniques such as code injection and memory manipulation are less trivial. It allows for experimenting with different approaches as there is no single way that's clearly better than the rest.

These factors make Bullet Force a fun target to reverse engineer and write tooling for. It's unlike games that cheats are traditionally written for like Assault Cube or Counter-Strike 2 where the same techniques carry over between games, and to the author's knowledge had not seen any open source cheats until the original release of [BulletForceHax][bfhaxv1].

[BulletForceHaxV2][bfhaxv2] as since succeeded [BulletForceHax][bfhaxv1], and later [BulletForceHaxV3][bfhaxv3] as succeeded it, which is where current focus lies. This book won't cover old iterations of BulletForceHax and you shouldn't expect any further development or support for them.

[pun]: https://www.photonengine.com/pun
[blayze]: https://blayzegames.com/
[bfweb]: https://www.crazygames.com/game/bullet-force-multiplayer
[bfandroid]: https://play.google.com/store/apps/details?id=com.blayzegames.iosfps
[bfios]: https://apps.apple.com/us/app/bullet-force/id1009134067
[bfsteam]: https://store.steampowered.com/app/450240/Bullet_Force/
[bfhaxv1]: https://github.com/holly-hacker/BulletForceHax
[bfhaxv2]: https://github.com/holly-hacker/BulletForceHaxV2
[bfhaxv3]: https://github.com/holly-hacker/BulletForceHaxV3
