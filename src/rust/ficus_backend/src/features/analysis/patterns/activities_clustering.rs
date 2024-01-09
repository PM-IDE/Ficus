use linfa::{prelude::Predict, traits::Transformer};

use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

use super::activity_instances::ActivityInTraceInfo;
use crate::{
    event_log::core::event::event_hasher::RegexEventHasher,
    utils::dataset::dataset::{FicusDataset, LabeledDataset},
};
use crate::{
    event_log::core::{event::event::Event, event_log::EventLog, trace::trace::Trace},
    features::analysis::patterns::repeat_sets::ActivityNode,
    pipelines::aliases::TracesActivities,
};
use linfa::metrics::SilhouetteScore;
use linfa::{traits::Fit, DatasetBase};
use linfa_clustering::{Dbscan, KMeans};
use linfa_nn::{distance::Distance, KdTree};
use ndarray::{Array1, Array2, ArrayBase, ArrayView, Dim, Dimension, OwnedRepr};
