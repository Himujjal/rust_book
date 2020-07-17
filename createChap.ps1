param (
  [switch] $Lib
)

$packageName = $args[0] ;

if ($Lib) {
  cargo new $packageName --lib ;
} else {  
  cargo new $packageName ;
}

Set-Location $packageName ;
nodemon -e rs,toml --exec "cls && cargo run" --watch "src/*.rs" --watch cargo.toml