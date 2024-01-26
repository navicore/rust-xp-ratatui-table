use itertools::Itertools;

#[derive(Clone, Debug)]
pub struct Container {
    pub name: String,
    pub description: String,
}

impl Container {
    pub(crate) const fn ref_array(&self) -> [&String; 2] {
        [&self.name, &self.description]
    }

    pub(crate) fn container(&self) -> &str {
        &self.name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }
}
pub fn generate_container_recs() -> Vec<Container> {
    use fakeit::generator;

    (0..2)
        .map(|_| {
            let container = generator::generate("???????????".to_string());
            let description = "Pod Container".to_string();

            Container {
                name: container,
                description,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect_vec()
}

#[derive(Clone, Debug)]
pub struct Pod {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) age: String,
    pub(crate) containers: String,
}

impl Pod {
    pub(crate) const fn ref_array(&self) -> [&String; 4] {
        [
            &self.name,
            &self.description,
            &self.age,
            &self.containers,
        ]
    }

    pub(crate) fn podname(&self) -> &str {
        &self.name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn age(&self) -> &str {
        &self.age
    }

    pub(crate) fn containers(&self) -> &str {
        &self.containers
    }
}
pub fn generate_pod_recs() -> Vec<Pod> {
    use fakeit::generator;

    (0..20)
        .map(|_| {
            let podname = generator::generate("replica###-??#?#?##-??#?#?#".to_string());
            let description = "Deployment Pod".to_string();
            let age = "200d".to_string();
            let containers = "2/2".to_string();

            Pod {
                name: podname,
                description,
                age,
                containers,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect_vec()
}
#[derive(Clone, Debug)]
pub struct Rs {
    pub(crate) name: String,
    pub(crate) description: String,
    pub(crate) age: String,
    pub(crate) pods: String,
    pub(crate) containers: String,
}

impl Rs {
    pub(crate) const fn ref_array(&self) -> [&String; 5] {
        [
            &self.name,
            &self.description,
            &self.age,
            &self.pods,
            &self.containers,
        ]
    }

    pub(crate) fn replicaset(&self) -> &str {
        &self.name
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn age(&self) -> &str {
        &self.age
    }

    pub(crate) fn pods(&self) -> &str {
        &self.pods
    }

    pub(crate) fn containers(&self) -> &str {
        &self.containers
    }
}
pub fn generate_rs_recs() -> Vec<Rs> {
    use fakeit::generator;

    (0..20)
        .map(|_| {
            let replicaset = generator::generate("replica###-??#?#?##".to_string());
            let description = "Deployment".to_string();
            let age = "200d".to_string();
            let pods = "4/4".to_string();
            let containers = "8/8".to_string();

            Rs {
                name: replicaset,
                description,
                age,
                pods,
                containers,
            }
        })
        .sorted_by(|a, b| a.name.cmp(&b.name))
        .collect_vec()
}