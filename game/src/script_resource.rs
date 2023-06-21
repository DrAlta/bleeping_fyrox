use std::{
    any::Any,
    borrow::Cow,
    path::{Path, PathBuf},
};



use fyrox::{
    asset::{
        event::ResourceEventBroadcaster,
        loader::{BoxedLoaderFuture, ResourceLoader},
        untyped::UntypedResource, ResourceData,
    }, core::{reflect::Reflect, TypeUuidProvider, uuid::{Uuid, uuid}, visitor::{Visit, VisitResult, Visitor}, reflect::prelude::*, io,},
};

use crate::script::Scripts;

#[derive(Debug, Visit, Reflect)]
pub struct ScriptResource {
    // You resource must store the path.
    path: PathBuf,
    pub some_data: Scripts,
}
impl ScriptResource {
    pub fn test(){
    }
}

impl TypeUuidProvider for ScriptResource {
    // Every resource must provide a unique identifier, that is used for dynamic type
    // casting, serialization, etc.
    fn type_uuid() -> Uuid {
        uuid!("f51f8911-3435-4053-958f-c57f35b2c510")
    }
}

impl ResourceData for ScriptResource {
    fn path(&self) -> Cow<Path> {
        Cow::Borrowed(&self.path)
    }

    fn set_path(&mut self, path: PathBuf) {
        self.path = path;
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_uuid(&self) -> Uuid {
        <Self as TypeUuidProvider>::type_uuid()
    }
}

pub struct ScriptResourceLoader;

impl ResourceLoader for ScriptResourceLoader {
    fn extensions(&self) -> &[&str] {
        // An array of extensitions, supported by this loader. There could be any number of extensions
        // since sometimes multiple extensions map to a single resource (for instance, jpg, png, bmp, are
        // all images).
        &["json"]
    }

    fn into_any(self: Box<Self>) -> Box<dyn Any> {
        self
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn load(
        &self,
        resource: UntypedResource,
        event_broadcaster: ResourceEventBroadcaster,
        reload: bool,
    ) -> BoxedLoaderFuture {
        Box::pin(async move {
            let path = resource.path();
            match io::load_file(&path).await {
                Ok(content) => {
                    let data = String::from_utf8(content).unwrap();
                    match serde_jsonrc::from_str::<Scripts>(&data) {
                        Ok(value)=> {
                            let my_resource = ScriptResource {
                                path,
                                some_data: value
                            };
                            resource.commit_ok(my_resource);

                            event_broadcaster.broadcast_loaded_or_reloaded(resource, reload);
                        }
                        Err(err) => {
                            resource.commit_error(path, err);
                        }
                    }
                }
                Err(err) => {
                    resource.commit_error(path, err);
                }
            }
        })
    }
}