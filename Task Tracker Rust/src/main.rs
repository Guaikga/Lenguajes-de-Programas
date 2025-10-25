mod tareas;
mod comandos;
mod guardado;

use comandos::configurar_cli;
use tareas::Tareas;
use guardado::cargar;

fn main() {
    let matches = configurar_cli().get_matches();

    let archivo = "tareas.json";
    let mut tareas = cargar(archivo);

    if !matches.args_present() {
        Tareas::mostrar_ayuda();
        return;
    }

    Tareas::procesar_comandos(matches, &mut tareas, archivo);
}