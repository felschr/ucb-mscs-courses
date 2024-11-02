use tonic::{Request, Response, Status};

use ucb_mscs_courses_proto::course::v1::courses_grpc_server::CoursesGrpc;
use ucb_mscs_courses_proto::course::v1::{ListCoursesRequest, ListCoursesResponse};

// use crate::repository::Repository;

pub struct MyCoursesGrpc {
    // pub repository: Repository,
}

#[tonic::async_trait]
impl CoursesGrpc for MyCoursesGrpc {
    async fn list_courses(
        &self,
        _request: Request<ListCoursesRequest>,
    ) -> Result<Response<ListCoursesResponse>, Status> {
        // let _ = self.repository.courses; // TODO

        Ok(Response::new(ListCoursesResponse {
            // TODO
            courses: vec![],
        }))
    }
}
