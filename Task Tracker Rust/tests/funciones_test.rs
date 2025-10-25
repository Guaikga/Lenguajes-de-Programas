use to_do_list::tareas::{Tareas, EstadoTarea};

#[test]
fn test_añadir_tarea_principal() {
    let mut tareas = Tareas::default();
    
    let id = tareas.añadir("Tarea de prueba".to_string());
    
    assert_eq!(id, 1);
    assert_eq!(tareas.lista.len(), 1);
    assert_eq!(tareas.lista[0].titulo, "Tarea de prueba");
    assert_eq!(tareas.lista[0].estado, EstadoTarea::Activa);
    assert_eq!(tareas.lista[0].numero_jerarquico, "1");
}

#[test]
fn test_añadir_subtarea() {
    let mut tareas = Tareas::default();
    let padre_id = tareas.añadir("Tarea principal".to_string());
    let resultado = tareas.añadir_subtarea(padre_id, "Subtarea".to_string());
    
    assert!(resultado.is_ok());
    let subtarea_id = resultado.unwrap();
    assert_eq!(subtarea_id, 2);
    
    let padre_index = tareas.buscar_por_id(padre_id).unwrap();
    assert_eq!(tareas.lista[padre_index].subtareas.len(), 1);
    assert_eq!(tareas.lista[padre_index].subtareas[0].titulo, "Subtarea");
    assert_eq!(tareas.lista[padre_index].subtareas[0].numero_jerarquico, "1.1");
}

#[test]
fn test_añadir_subtarea_a_tarea_inexistente() {
    let mut tareas = Tareas::default();
    
    let resultado = tareas.añadir_subtarea(999, "Subtarea".to_string());
    
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), "ID 999 no encontrado");
}

#[test]
fn test_buscar_por_id() {
    let mut tareas = Tareas::default();
    
    let id = tareas.añadir("Tarea de prueba".to_string());
    
    let resultado = tareas.buscar_por_id(id);
    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), 0);
    
    let resultado_no_existe = tareas.buscar_por_id(999);
    assert!(resultado_no_existe.is_err());
}

#[test]
fn test_editar_tarea() {
    let mut tareas = Tareas::default();
    
    let id = tareas.añadir("Tarea original".to_string());
    
    let resultado = tareas.editar(id, "Tarea editada".to_string());
    
    assert!(resultado.is_ok());
    assert_eq!(tareas.lista[0].titulo, "Tarea editada");
}

#[test]
fn test_editar_tarea_inexistente() {
    let mut tareas = Tareas::default();
    
    let resultado = tareas.editar(999, "Tarea editada".to_string());
    
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), "ID 999 no encontrado");
}

#[test]
fn test_eliminar_tarea_principal() {
    let mut tareas = Tareas::default();
    
    let id = tareas.añadir("Tarea a eliminar".to_string());
    assert_eq!(tareas.lista.len(), 1);
    
    let resultado = tareas.eliminar(id);
    
    assert!(resultado.is_ok());
    assert_eq!(tareas.lista.len(), 0);
}

#[test]
fn test_eliminar_subtarea() {
    let mut tareas = Tareas::default();
    
    let padre_id = tareas.añadir("Tarea principal".to_string());
    let subtarea_id = tareas.añadir_subtarea(padre_id, "Subtarea".to_string()).unwrap();
    
    let padre_index = tareas.buscar_por_id(padre_id).unwrap();
    assert_eq!(tareas.lista[padre_index].subtareas.len(), 1);
    
    let resultado = tareas.eliminar(subtarea_id);
    
    assert!(resultado.is_ok());
    assert_eq!(tareas.lista[padre_index].subtareas.len(), 0);
}

#[test]
fn test_eliminar_tarea_inexistente() {
    let mut tareas = Tareas::default();
    
    let resultado = tareas.eliminar(999);
    
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), "ID 999 no encontrado");
}

#[test]
fn test_cambiar_estado_tarea() {
    let mut tareas = Tareas::default();
    
    let id = tareas.añadir("Tarea de prueba".to_string());
    assert_eq!(tareas.lista[0].estado, EstadoTarea::Activa);
    
    let resultado = tareas.set_estado(id, EstadoTarea::Finalizada);
    
    assert!(resultado.is_ok());
    assert_eq!(tareas.lista[0].estado, EstadoTarea::Finalizada);
    assert!(tareas.lista[0].finalizada_en.is_some());
}

#[test]
fn test_cambiar_estado_tarea_inexistente() {
    let mut tareas = Tareas::default();
    
    let resultado = tareas.set_estado(999, EstadoTarea::Finalizada);
    
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), "ID 999 no encontrado");
}

#[test]
fn test_estado_automatico_por_subtareas() {
    let mut tareas = Tareas::default();
    
    let padre_id = tareas.añadir("Tarea principal".to_string());

    let subtarea1_id = tareas.añadir_subtarea(padre_id, "Subtarea 1".to_string()).unwrap();
    let subtarea2_id = tareas.añadir_subtarea(padre_id, "Subtarea 2".to_string()).unwrap();
    
    let padre_index = tareas.buscar_por_id(padre_id).unwrap();
    assert_eq!(tareas.lista[padre_index].estado, EstadoTarea::Activa);
    
    tareas.set_estado(subtarea1_id, EstadoTarea::Finalizada).unwrap();
    assert_eq!(tareas.lista[padre_index].estado, EstadoTarea::Activa);
    
    tareas.set_estado(subtarea2_id, EstadoTarea::Finalizada).unwrap();
    assert_eq!(tareas.lista[padre_index].estado, EstadoTarea::Finalizada);
}

#[test]
fn test_listar_subtareas() {
    let mut tareas = Tareas::default();
    
    let padre_id = tareas.añadir("Tarea principal".to_string());
    tareas.añadir_subtarea(padre_id, "Subtarea 1".to_string()).unwrap();
    tareas.añadir_subtarea(padre_id, "Subtarea 2".to_string()).unwrap();
    
    let resultado = tareas.listar_subtareas(padre_id);
    
    assert!(resultado.is_ok());
}

#[test]
fn test_listar_subtareas_tarea_sin_subtareas() {
    let mut tareas = Tareas::default();
    
    let padre_id = tareas.añadir("Tarea sin subtareas".to_string());
    
    let resultado = tareas.listar_subtareas(padre_id);
    
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("no tiene subtareas"));
}

#[test]
fn test_listar_subtareas_tarea_inexistente() {
    let tareas = Tareas::default();
    
    let resultado = tareas.listar_subtareas(999);
    
    assert!(resultado.is_err());
    assert_eq!(resultado.unwrap_err(), "ID 999 no encontrado");
}

#[test]
fn test_validar_formato_dos_partes() {
    let resultado = Tareas::validar_formato_dos_partes("edit", "1:Nuevo título", "editar tarea");
    
    assert!(resultado.is_ok());
    let (id, texto) = resultado.unwrap();
    assert_eq!(id, 1);
    assert_eq!(texto, "Nuevo título");
}

#[test]
fn test_validar_formato_dos_partes_invalido() {
    let resultado = Tareas::validar_formato_dos_partes("edit", "1", "editar tarea");
    
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("Formato incorrecto"));
}

#[test]
fn test_validar_formato_estado() {
    let resultado = Tareas::validar_formato_estado("1:1");
    
    assert!(resultado.is_ok());
    let (id, estado) = resultado.unwrap();
    assert_eq!(id, 1);
    assert_eq!(estado, EstadoTarea::Finalizada);
}

#[test]
fn test_validar_formato_estado_invalido() {
    let resultado = Tareas::validar_formato_estado("1:5");
    
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("Estado '5' inválido"));
}

#[test]
fn test_validar_id_simple() {
    let resultado = Tareas::validar_id_simple("delete", "1");
    
    assert!(resultado.is_ok());
    assert_eq!(resultado.unwrap(), 1);
}

#[test]
fn test_validar_id_simple_invalido() {
    let resultado = Tareas::validar_id_simple("delete", "abc");
    
    assert!(resultado.is_err());
    assert!(resultado.unwrap_err().contains("no es un número válido"));
}

#[test]
fn test_todas_subtareas_completadas() {
    let mut tarea = to_do_list::tareas::modelo::Tarea {
        id: 1,
        titulo: "Tarea prueba".to_string(),
        estado: EstadoTarea::Activa,
        inicio: chrono::Local::now(),
        finalizada_en: None,
        subtareas: Vec::new(),
        es_subtarea: false,
        padre_id: None,
        numero_jerarquico: "1".to_string(),
    };

    assert!(tarea.todas_subtareas_completadas());
    
    let subtarea_activa = to_do_list::tareas::modelo::Tarea {
        id: 2,
        titulo: "Subtarea activa".to_string(),
        estado: EstadoTarea::Activa,
        inicio: chrono::Local::now(),
        finalizada_en: None,
        subtareas: Vec::new(),
        es_subtarea: true,
        padre_id: Some(1),
        numero_jerarquico: "1.1".to_string(),
    };
    
    tarea.subtareas.push(subtarea_activa);
    assert!(!tarea.todas_subtareas_completadas());
    
    tarea.subtareas[0].estado = EstadoTarea::Finalizada;
    assert!(tarea.todas_subtareas_completadas());
}