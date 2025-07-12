// Smart Contract Registry Builder
// Author: azaM & Copilot 🛠️
// Concepts: Typestate, Rc<RefCell>, HashMap metadata, FnOnce hook, fluent chaining

use std::{cell::RefCell, collections::HashMap, rc::Rc};

// Typestate markers
struct Init;
struct Validated;
struct Deployed;

type Metadata = Rc<RefCell<HashMap<String, String>>>;

struct ContractBuilder<State> {
    name: String,
    metadata: Metadata,
    _state: std::marker::PhantomData<State>,
}

impl ContractBuilder<Init> {
    fn new(name: &str) -> Self {
        Self {
            name: name.into(),
            metadata: Rc::new(RefCell::new(HashMap::new())),
            _state: std::marker::PhantomData,
        }
    }

    fn with_author(self, author: &str) -> Self {
        self.metadata
            .borrow_mut()
            .insert("author".into(), author.into());
        self
    }

    fn validate(self) -> ContractBuilder<Validated> {
        self.metadata
            .borrow_mut()
            .insert("validated".into(), "true".into());
        ContractBuilder {
            name: self.name,
            metadata: self.metadata,
            _state: std::marker::PhantomData,
        }
    }
}

impl ContractBuilder<Validated> {
    fn on_deploy<F>(self, hook: F) -> ContractBuilder<Deployed>
    where
        F: FnOnce(&mut HashMap<String, String>),
    {
        self.metadata
            .borrow_mut()
            .insert("status".into(), "deployed".into());
        {
            let mut meta = self.metadata.borrow_mut();
            hook(&mut meta); // deploy-time logic (e.g. timestamp, signer)
        }

        ContractBuilder {
            name: self.name,
            metadata: self.metadata,
            _state: std::marker::PhantomData,
        }
    }
}

impl ContractBuilder<Deployed> {
    fn registry(self) -> Metadata {
        self.metadata
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn metadata(&self) -> Rc<RefCell<HashMap<String, String>>> {
        Rc::clone(&self.metadata)
    }

    fn borrow(&self) -> std::cell::Ref<HashMap<String, String>> {
        self.metadata.borrow()
    }

    fn borrow_mut(&self) -> std::cell::RefMut<HashMap<String, String>> {
        self.metadata.borrow_mut()
    }

    fn into_inner(self) -> HashMap<String, String> {
        Rc::try_unwrap(self.metadata)
            .ok()
            .map(|rc| rc.into_inner())
            .unwrap_or_else(HashMap::new)
    }

    fn into_deployed(self) -> Deployed {
        Deployed {
            name: self.name,
            metadata: self.metadata,
            _state: std::marker::PhantomData,
        }
    }
}

fn main() {
    let registry = ContractBuilder::new("TokenX")
        .with_author("azaM")
        .validate()
        .on_deploy(|meta| {
            meta.insert("timestamp".into(), "2025-06-28".into());
            meta.insert("signer".into(), "0xDEADBEEF".into());
        })
        .registry();

    println!("📘 Contract Metadata:");
    for (k, v) in registry.borrow().iter() {
        println!("  {k}: {v}");
    }
}

