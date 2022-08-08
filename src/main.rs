#![no_std]
// desativa a biblioteca standard
#![no_main]
// Diz para o compilador que não será a usadado o entrypoint padrão. 
// o padrão é: crt0 (C runtime zero) -> start language item (rust runtime) -> main function.

use core::panic::PanicInfo;

/// Entrypoint:
#[no_mangle]
pub extern "C" fn _start() -> !{
    //! 
    //! [#[no_mangle]]: Faz o compilador manter o nome _start. Caso o contrário ele 
    //! geraria um nome único tipo "_ZN3blog_os4_start7hb173fedf945531caE".
    //! 
    //! [extern "C"]: Diz para o compilador usar a C calling convention para essa função no 
    //! lugar da Rust calling convention.
    //!
    //! [_start]: É o nome do entrypoint na maioria dos sistemas.
    //! 
    //! [!]: Indica que a funcão é do tipo "diverging". Não permite retorno.
    //! Isso é necessário porque o entry point não é chamado por uma função, 
    //! mas pelo SO ou bootloader. Então ao invés de return, é invocado o system call "exit" do SO.
    //! No nosso caso, desligar o computador seria o suficiente, pois não há o que fazer com o 
    //! retorno do nosso binário. (Por ser um SO).
    
    loop {}
}

/// Panic Handler:
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //! É preciso recriá-lo por não usarmos a std.
    //! 
    //! [!]: Indica que a funcão é do tipo "diverging". Não permite retorno.
    //! Isso é necessário porque o entry point não é chamado por uma função, 
    //! mas pelo SO ou bootloader. Então ao invés de return, é invocado o system call "exit" do SO.
    //! No nosso caso, desligar o computador seria o suficiente, pois não há o que fazer com o 
    //! retorno do nosso binário. (Por ser um SO).

    loop {}
}

