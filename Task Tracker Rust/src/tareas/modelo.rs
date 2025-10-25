use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EstadoTarea {
    Activa,
    Finalizada,
    Suspendida,
}

impl std::fmt::Display for EstadoTarea {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let estado = match self {
            EstadoTarea::Activa => "Activa",
            EstadoTarea::Finalizada => "Finalizada",
            EstadoTarea::Suspendida => "Suspendida",
        };
        write!(f, "{}", estado)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tarea {
    pub id: u32,
    pub titulo: String,
    pub estado: EstadoTarea,
    pub inicio: DateTime<Local>,
    pub finalizada_en: Option<DateTime<Local>>,
    pub subtareas: Vec<Tarea>,
    pub es_subtarea: bool,
    pub padre_id: Option<u32>,
    pub numero_jerarquico: String,
}

impl Tarea {
    pub fn todas_subtareas_completadas(&self) -> bool {
        if self.subtareas.is_empty() {
            return true;
        }
        self.subtareas.iter().all(|t| t.estado == EstadoTarea::Finalizada)
    }

    pub fn alguna_subtarea_activa(&self) -> bool {
        self.subtareas.iter().any(|t| t.estado == EstadoTarea::Activa)
    }

    pub fn alguna_subtarea_suspendida(&self) -> bool {
        self.subtareas.iter().any(|t| t.estado == EstadoTarea::Suspendida)
    }

    pub fn actualizar_estado_por_subtareas(&mut self) {
        if self.subtareas.is_empty() {
            return;
        }

        if self.todas_subtareas_completadas() {
            self.estado = EstadoTarea::Finalizada;
            self.finalizada_en = Some(Local::now());
        } else if self.alguna_subtarea_suspendida() {
            self.estado = EstadoTarea::Suspendida;
            self.finalizada_en = None;
        } else if self.alguna_subtarea_activa() {
            self.estado = EstadoTarea::Activa;
            self.finalizada_en = None;
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Tareas {
    pub lista: Vec<Tarea>,
    pub ultimo_id: u32,
    pub contador_principal: u32,
}