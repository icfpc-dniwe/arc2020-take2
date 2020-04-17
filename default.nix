{ lib, xclip, rustPlatform }:

rustPlatform.buildRustPackage {
  name = "arc2020-take2";
  version = "0.1";

  src = lib.cleanSource ./.;

  cargoSha256 = "0rq2brpii5vf54h5va5f7kjyjm6jgk3gxzfic4j6fp2irkjr7a2g";

  buildInputs = [ ];
  nativeBuildInputs = [ xclip ];

  doCheck = false;

  postInstall = ''
    mv $out/bin $out/lib
  '';
}
