use std::fmt;
use std::ops::Deref;
use std::sync::Arc;
use typed_builder::TypedBuilder;

use crate::core::{DependencyVersionReq, PackageId, PackageName, SourceId, Summary, TargetKind};

/// See [`ManifestDependencyInner`] for public fields reference.
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct ManifestDependency(Arc<ManifestDependencyInner>);

#[derive(TypedBuilder, Clone, Eq, PartialEq, Hash)]
#[builder(builder_type(name = ManifestDependencyBuilder))]
#[builder(builder_method(vis = ""))]
#[builder(build_method(into = ManifestDependency))]
pub struct ManifestDependencyInner {
    pub name: PackageName,
    pub version_req: DependencyVersionReq,
    #[builder(default)]
    pub source_id: SourceId,
    #[builder(default)]
    pub kind: DepKind,
}

#[derive(Clone, Default, Eq, PartialEq, Hash)]
pub enum DepKind {
    #[default]
    Normal,
    Target(TargetKind),
}

impl Deref for ManifestDependency {
    type Target = ManifestDependencyInner;

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

#[doc(hidden)]
impl From<ManifestDependencyInner> for ManifestDependency {
    fn from(data: ManifestDependencyInner) -> Self {
        Self(Arc::new(data))
    }
}

impl ManifestDependency {
    pub fn builder() -> ManifestDependencyBuilder {
        ManifestDependencyInner::builder()
    }

    pub fn matches_summary(&self, summary: &Summary) -> bool {
        self.matches_package_id(summary.package_id)
    }

    pub fn matches_package_id(&self, package_id: PackageId) -> bool {
        package_id.name == self.name && self.version_req.matches(&package_id.version)
    }
}

impl fmt::Display for ManifestDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name, self.version_req)?;

        if !self.source_id.is_default_registry() {
            write!(f, " ({})", self.source_id)?;
        }

        Ok(())
    }
}

impl fmt::Debug for ManifestDependency {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ManifestDependency({self})")
    }
}
