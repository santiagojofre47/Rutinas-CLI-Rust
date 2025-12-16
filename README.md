# Rutinas-CLI-Rust

Este es mi primer script en Rust, su principal objetivo es simular algunas rutinas CLI que realiza el S.O mediante el shell (Aclaración: no es un S.O ni pretende serlo, solamente busca simular el uso de rutinas CLI).

Modo de uso: ejecutar en una terminal "cargo run -- -[comando]"
Los comandos implementados hasta ahora:
-add [args], requiere mínimo un operando
-sub [args], requiere mínimo un operando
-mult [args], requiere mínimo un operando
-div [args], requiere mínimo un operando
-odd [arg]
-equal [arg], compara cadenas
-echo [texto]
-wc [nombre de archivo de texto], muestra la información de un archivo de texto (Cantidad de lineas, palabras y bytes)
-echo_file [nombre de archivo de texto, cadena, nueva] busca en el archivo de texto una cadena y la reescribe por una cadena nueva
-concatenate [args] concatena dos o mas cadenas
-stat [nombre del archivo] simula el comando de Linux -stat mostrando algunos metadatos del archivo leído
