use crate::tareas::{EstadoTarea, Tareas};
use crate::guardado;
use clap::{Arg, ArgAction, Command, ArgMatches};

pub fn configurar_cli() -> Command {
    Command::new("Task Tracker")
        .about("Sistema de gestión de tareas con subtareas")
        .arg(Arg::new("add").long("add").value_name("TEXTO").num_args(0..=1).help("Añade una nueva tarea principal"))
        .arg(Arg::new("addsub").long("addsub").value_name("ID_PADRE:TEXTO").num_args(0..=1).help("Añade una subtarea a una tarea existente"))
        .arg(Arg::new("edit").long("edit").value_name("ID:TEXTO").num_args(0..=1).help("Edita una tarea"))
        .arg(Arg::new("delete").long("delete").value_name("ID").num_args(0..=1).help("Elimina una tarea"))
        .arg(Arg::new("setestado").long("setestado").value_name("ID:ESTADO").num_args(0..=1).help("Cambia el estado de una tarea (0=Activa, 1=Finalizada, 2=Suspendida)"))
        .arg(Arg::new("list").long("list").action(ArgAction::SetTrue).help("Lista todas las tareas con formato jerárquico"))
        .arg(Arg::new("listsub").long("listsub").value_name("ID").num_args(0..=1).help("Lista las subtareas de una tarea específica"))
        .arg(Arg::new("listactivas").long("listactivas").action(ArgAction::SetTrue).help("Lista solo tareas activas"))
        .arg(Arg::new("listsuspendidas").long("listsuspendidas").action(ArgAction::SetTrue).help("Lista solo tareas suspendidas"))
        .arg(Arg::new("listfinalizadas").long("listfinalizadas").action(ArgAction::SetTrue).help("Lista solo tareas finalizadas"))
}

impl Tareas {
    pub fn mostrar_ayuda() {
        println!("\nCOMANDOS DISPONIBLES:");
        println!("  --add TEXTO                    Añadir nueva tarea principal");
        println!("  --addsub ID_PADRE:TEXTO        Añadir subtarea a una tarea existente");
        println!("  --edit ID:TEXTO                Editar título de una tarea");
        println!("  --delete ID                    Eliminar una tarea");
        println!("  --setestado ID:ESTADO          Cambiar estado (0=Activa, 1=Finalizada, 2=Suspendida)");
        println!("  --list                         Listar todas las tareas");
        println!("  --listsub ID                   Listar subtareas de una tarea");
        println!("  --listactivas                  Listar solo tareas activas");
        println!("  --listfinalizadas              Listar solo tareas finalizadas");
        println!("  --listsuspendidas              Listar solo tareas suspendidas");
    }

    pub fn validar_formato_dos_partes(comando: &str, valor: &str, nombre_comando: &str) -> Result<(u32, String), String> {
        let partes: Vec<&str> = valor.splitn(2, ':').collect();
        if partes.len() != 2 {
            return Err(format!(
                "Error: Formato incorrecto para {}\nUso: --{} ID:TEXTO\nEjemplo: --{} 1:\"Nuevo título\"", 
                nombre_comando, comando, comando
            ));
        }
        
        let id = partes[0].parse::<u32>()
            .map_err(|_| format!("Error: ID '{}' no es un número válido", partes[0]))?;
        
        if partes[1].trim().is_empty() {
            return Err(format!("Error: El texto no puede estar vacío"));
        }
        
        Ok((id, partes[1].to_string()))
    }

    pub fn validar_formato_estado(valor: &str) -> Result<(u32, EstadoTarea), String> {
        let partes: Vec<&str> = valor.splitn(2, ':').collect();
        if partes.len() != 2 {
            return Err(format!(
                "Error: Formato incorrecto para setestado\nUso: --setestado ID:ESTADO\nEstados: 0=Activa, 1=Finalizada, 2=Suspendida\nEjemplo: --setestado 1:1"
            ));
        }
        
        let id = partes[0].parse::<u32>()
            .map_err(|_| format!("Error: ID '{}' no es un número válido", partes[0]))?;
        
        let estado = match partes[1].parse::<u8>() {
            Ok(0) => EstadoTarea::Activa,
            Ok(1) => EstadoTarea::Finalizada,
            Ok(2) => EstadoTarea::Suspendida,
            Ok(n) => return Err(format!("Error: Estado '{}' inválido. Use 0, 1 o 2", n)),
            Err(_) => return Err(format!("Error: Estado '{}' no es un número válido", partes[1])),
        };
        
        Ok((id, estado))
    }

    pub fn validar_id_simple(comando: &str, valor: &str) -> Result<u32, String> {
        if valor.trim().is_empty() {
            return Err(format!(
                "Error: Falta el ID para {}\nUso: --{} ID\nEjemplo: --{} 1", 
                comando, comando, comando
            ));
        }
        
        valor.parse::<u32>()
            .map_err(|_| format!("Error: ID '{}' no es un número válido", valor))
    }

    pub fn procesar_comandos(matches: ArgMatches, tareas: &mut Tareas, archivo: &str) {
        if matches.contains_id("add") {
            match matches.get_one::<String>("add") {
                Some(titulo) => {
                    if titulo.trim().is_empty() {
                        println!("Error: El título de la tarea no puede estar vacío");
                        println!("Uso: --add \"Título de la tarea\"");
                    } else {
                        let _id = tareas.añadir(titulo.to_string());
                        println!("Tarea {} añadida: {}", tareas.contador_principal, titulo);
                    }
                },
                None => {
                    println!("Error: Falta el título para --add");
                    println!("Uso: --add \"Título de la tarea\"");
                    println!("Ejemplo: --add \"Mi nueva tarea\"");
                }
            }
        }

        if matches.contains_id("addsub") {
            match matches.get_one::<String>("addsub") {
                Some(addsub) => {
                    match Tareas::validar_formato_dos_partes("addsub", addsub, "añadir subtarea") {
                        Ok((padre_id, titulo)) => {
                            match tareas.añadir_subtarea(padre_id, titulo.clone()) {
                                Ok(id) => {
                                    if let Ok(padre_index) = tareas.buscar_por_id(padre_id) {
                                        if let Some(subtarea) = tareas.lista[padre_index].subtareas.iter().find(|st| st.id == id) {
                                            println!("Subtarea {} (ID: {}) añadida: {}", subtarea.numero_jerarquico, id, titulo);
                                        }
                                    }
                                },
                                Err(e) => println!("Error: {}", e),
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                None => {
                    println!("Error: Falta el valor para --addsub");
                    println!("Uso: --addsub ID_PADRE:TEXTO");
                    println!("Ejemplo: --addsub 1:\"Mi subtarea\"");
                }
            }
        }

        if matches.contains_id("edit") {
            match matches.get_one::<String>("edit") {
                Some(edit) => {
                    match Tareas::validar_formato_dos_partes("edit", edit, "editar tarea") {
                        Ok((id, nuevo_titulo)) => {
                            if let Err(e) = tareas.editar(id, nuevo_titulo.clone()) {
                                println!("Error: {}", e);
                            } else {
                                println!("Tarea #{} editada: {}", id, nuevo_titulo);
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                None => {
                    println!("Error: Falta el valor para --edit");
                    println!("Uso: --edit ID:TEXTO");
                    println!("Ejemplo: --edit 1:\"Nuevo título\"");
                }
            }
        }

        if matches.contains_id("delete") {
            match matches.get_one::<String>("delete") {
                Some(delete) => {
                    match Tareas::validar_id_simple("delete", delete) {
                        Ok(id) => {
                            if let Err(e) = tareas.eliminar(id) {
                                println!("Error: {}", e);
                            } else {
                                println!("Tarea #{} eliminada", id);
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                None => {
                    println!("Error: Falta el ID para --delete");
                    println!("Uso: --delete ID");
                    println!("Ejemplo: --delete 1");
                }
            }
        }

        if matches.contains_id("setestado") {
            match matches.get_one::<String>("setestado") {
                Some(setestado) => {
                    match Tareas::validar_formato_estado(setestado) {
                        Ok((id, estado)) => {
                            if let Err(e) = tareas.set_estado(id, estado) {
                                println!("Error: {}", e);
                            } else {
                                println!("Estado de tarea #{} cambiado a {}", id, estado);
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                None => {
                    println!("Error: Falta el valor para --setestado");
                    println!("Uso: --setestado ID:ESTADO");
                    println!("Estados: 0=Activa, 1=Finalizada, 2=Suspendida");
                    println!("Ejemplo: --setestado 1:1");
                }
            }
        }

        if matches.get_flag("list") {
            tareas.listar();
        }

        if matches.contains_id("listsub") {
            match matches.get_one::<String>("listsub") {
                Some(listsub) => {
                    match Tareas::validar_id_simple("listsub", listsub) {
                        Ok(padre_id) => {
                            if let Err(e) = tareas.listar_subtareas(padre_id) {
                                println!("Error: {}", e);
                            }
                        },
                        Err(e) => println!("{}", e),
                    }
                },
                None => {
                    println!("Error: Falta el ID para --listsub");
                    println!("Uso: --listsub ID");
                    println!("Ejemplo: --listsub 1");
                }
            }
        }

        if matches.get_flag("listactivas") {
            tareas.listar_por_estado(EstadoTarea::Activa);
        }

        if matches.get_flag("listsuspendidas") {
            tareas.listar_por_estado(EstadoTarea::Suspendida);
        }

        if matches.get_flag("listfinalizadas") {
            tareas.listar_por_estado(EstadoTarea::Finalizada);
        }

        if let Err(e) = guardado::guardar(tareas, archivo) {
            println!("Error al guardar: {}", e);
        }
    }
}