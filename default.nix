{ lib, xclip, rustPlatform }:

rustPlatform.buildRustPackage {
  name = "arc2020-take2";
  version = "0.1";

  src = lib.cleanSource ./.;

  cargoSha256 = "0psdjgcnabnydqai3rj9fdxh1rw5wv3lybgj41l0iy8sn9rm0hzd";

  buildInputs = [ ];
  nativeBuildInputs = [ xclip ];

  doCheck = false;

  postInstall = ''
    mv $out/bin $out/lib
  '';
}
