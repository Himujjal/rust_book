param ($packageName, [switch] $Lib, [switch] $Code);

if ($Lib) {
  cargo new $packageName --lib ;
}
else {  
  cargo new $packageName ;
}

Set-Location $packageName ;

if ($code) {
  code .
}

if (!$Lib) {
  nodemon -e rs, toml --exec "cls && cargo fmt && cargo run" --watch "src/*.rs" --watch cargo.toml
}
