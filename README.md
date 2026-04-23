# Pearchat67
esta cosa es un chat local para la red local, diseñado especificamente para los tontillos que quieren platicar en clase de informatica pero la maestra les regaña asi que pues con esto pueden ser felices.
Ademas este proyecto es como una practica de rust pero pues bueno, algo como un 30-minutes project

# ¿Es "seguro"?
No, ni de chiste es seguro, no lo uses si tu objetivo es un chat ultra-seguro, si buscas eso, este no es lo que buscas, el objetivo no es la seguridad del chat, es simplemente cumplir el objetivo que es hablar, ningun mensaje va cifrado, cualquiera con wireshark lo lee. Aunque esto no quita la posibilidad de que algun dia ponga encriptacion con HTTPS o alguna cosa asi

# Caracteristicas :
- Es estupidamente pequeño, probablemente NUNCA pase de 1000 lineas! (sin contar obviamente, librerias)

# Como perejiles se instala
Solo ocupas tener instalado git y rust. Nada mas, casi todas las distros ya lo traen por defecto, o si te crees especial y usas arch lo mas probable es que tambien lo tengas igual.
Despues de tener eso solo copea los siguientes comandos y ya es todo
git clone https://github.com/SrTortugardo/Pearchat.git
cd Pearchat
cargo build
cargo install --path .

recuerda tener ~/.cargo/bin/ en tu path o si usas Microsoft Windows C:\Users\TU_USUARIO\.cargo\bin\
