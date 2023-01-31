------------------------------------------------------------------------------
 base_egui. Una aplicación multiplataforma con interfaz gráfica hecha en Rust
 ----------------------------------------------------------------------------

Basada en la plantilla eframe (https://github.com/emilk/eframe_template.git) con la biblioteca egui (https://github.com/emilk/egui.git).

Lista para poder ejecutarse tanto en nativo, en diferentes arquitecturas, como en un navegador web.

Preparada para compilarse en cualquier parte mediante un contenedor docker.

Pensada como una base para otras futuras aplicaciones.

Programada en puro Rust.



INSTRUCCIONES DE COMPILACIÓN:

    Como único requisito debemos tener instalado docker (y git si queremos clonar este repositorio). Nos situaremos en el directorio "compilacion/" y ejecutaremos el script "compilar_con_docker.sh". Esto creará un contenedor que generará los ejecutables "base_egui_linux_x86" y "base_egui_windows_x86.exe" en el directorio "compilacion/", así como los archivos de la versión web en el directorio "compilacion/version_web/". La primera vez que se haga, el proceso podrá tardar varios minutos. En un futuro se podría ampliar fácilmente este proyecto para compilar a otras arquitecturas diferentes.

    $ git clone https://github.com/p4bloOS/base_egui.git
    $ cd base_egui/compilacion
    $ sudo sh compilar_con_docker.sh
 

INSTRUCCIONES PARA ARRANCAR SERVIDOR WEB:

    Como único requisito debemos tener instalado docker (y git si queremos clonar este repositorio). Nos situaremos en el directorio "servidor_web/" y ejecutaremos el script "arrancar_servidor_con_docker.sh". Esto creará un contenedor donde se ejecutará el servidor, que escuchará en el puerto 8080 de nuestro sistema (dicho puerto se puede cambiar fácilmente editando el script "arrancar_servidor_con_docker.sh").

    $ git clone https://github.com/p4bloOS/base_egui.git
    $ cd base_egui/servidor_web
    $ sudo sh arrancar_servidor_con_docker.sh



OTRAS CUESTIONES:

    - Como novedad sobre la plantilla eframe se ha implementado una característica que establece un tamaño de fuente apropiado según la resolución del monitor donde se ejecute la aplicación.
    - Es posible compilar el proyecto y ejecutar el servicio web sin usar docker, pero es un proceso laborioso y puede crear algunos problemas.