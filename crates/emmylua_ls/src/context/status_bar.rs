use std::sync::Arc;

use lsp_types::{
    NumberOrString, ProgressParams, ProgressParamsValue, WorkDoneProgress, WorkDoneProgressBegin,
    WorkDoneProgressCreateParams, WorkDoneProgressEnd, WorkDoneProgressReport,
};
use serde::{Deserialize, Serialize};

use super::ClientProxy;

pub struct StatusBar {
    client: Arc<ClientProxy>,
}

#[derive(Clone, Copy)]
pub enum ProgressTask {
    LoadWorkspace = 0,
    DiagnoseWorkspace = 1,
    #[allow(dead_code)]
    RefreshIndex = 2,
}

impl ProgressTask {
    pub fn as_i32(&self) -> i32 {
        *self as i32
    }

    pub fn get_task_name(&self) -> &'static str {
        match self {
            ProgressTask::LoadWorkspace => "Load workspace",
            ProgressTask::DiagnoseWorkspace => "Diagnose workspace",
            ProgressTask::RefreshIndex => "Refresh index",
        }
    }
}

impl StatusBar {
    pub fn new(client: Arc<ClientProxy>) -> Self {
        Self { client }
    }

    pub fn create_progress_task(&self, task: ProgressTask) {
        self.client.send_request_no_response(
            "window/workDoneProgress/create",
            WorkDoneProgressCreateParams {
                token: NumberOrString::Number(task.as_i32()),
            },
        );
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task as i32),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::Begin(
                    WorkDoneProgressBegin {
                        title: task.get_task_name().to_string(),
                        cancellable: Some(false),
                        message: Some(task.get_task_name().to_string()),
                        percentage: None,
                    },
                )),
            },
        )
    }

    pub fn update_progress_task(
        &self,
        task: ProgressTask,
        percentage: Option<u32>,
        message: Option<String>,
    ) {
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task.as_i32()),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::Report(
                    WorkDoneProgressReport {
                        percentage,
                        cancellable: Some(false),
                        message,
                    },
                )),
            },
        )
    }

    pub fn finish_progress_task(&self, task: ProgressTask, message: Option<String>) {
        self.client.send_notification(
            "$/progress",
            ProgressParams {
                token: NumberOrString::Number(task.as_i32()),
                value: ProgressParamsValue::WorkDone(WorkDoneProgress::End(WorkDoneProgressEnd {
                    message,
                })),
            },
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmmyServerStatus {
    health: String,
    loading: bool,
    message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmmyProgress {
    text: String,
    percent: f64,
}
