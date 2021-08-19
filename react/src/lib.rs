use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::DerefMut;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type Callback<'a, T> = Box<RefCell<dyn FnMut(T) + 'a>>;

pub struct Reactor<'a, T>
where
    T: Copy + PartialEq,
{
    cell_id_counter: usize,
    callback_id_counter: usize,
    cells: Vec<(CellID, Option<CellID>)>,
    values: HashMap<CellID, T>,
    funcs: HashMap<CellID, Box<dyn Fn(&[T]) -> T>>,
    callbacks: Vec<(CellID, CallbackID, Callback<'a, T>, bool)>,
}

impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cell_id_counter: 0,
            callback_id_counter: 0,
            cells: vec![],
            values: HashMap::new(),
            funcs: HashMap::new(),
            callbacks: vec![],
        }
    }

    fn new_cell_id_inner(&mut self) -> usize {
        self.cell_id_counter += 1;
        self.cell_id_counter
    }

    fn new_callback_id_inner(&mut self) -> usize {
        self.callback_id_counter += 1;
        self.callback_id_counter
    }

    pub fn create_input(&mut self, initial: T) -> InputCellID {
        let input_cell_id = InputCellID(self.new_cell_id_inner());
        let cell_id = CellID::Input(input_cell_id);
        self.values.insert(cell_id, initial);
        self.cells.push((cell_id, None));
        input_cell_id
    }

    pub fn create_compute<F: Fn(&[T]) -> T>(
        &mut self,
        dependencies: &[CellID],
        compute_func: F,
    ) -> Result<ComputeCellID, CellID>
    where
        F: 'static,
    {
        let compute_cell_id = ComputeCellID(self.new_cell_id_inner());
        let cell_id = CellID::Compute(compute_cell_id);
        let mut v: Vec<T> = vec![];

        for x in dependencies {
            if let Some(value) = self.values.get(x) {
                v.push(*value);
            } else {
                return Err(*x);
            }
        }

        let value = compute_func(&v);
        self.values.insert(cell_id, value);

        for &x in dependencies {
            self.cells.push((x, Some(cell_id)));
        }

        self.funcs.insert(cell_id, Box::new(compute_func));

        Ok(compute_cell_id)
    }

    fn get_outputs(&self, input_cell_id: CellID) -> Vec<CellID> {
        self.cells
            .iter()
            .filter_map(|(i, o)| if *i == input_cell_id { *o } else { None })
            .collect()
    }

    fn get_dependencies_values(&self, output_cell_id: CellID) -> Vec<T> {
        self.cells
            .iter()
            .filter_map(|(i, o)| match *o {
                Some(id) => {
                    if id == output_cell_id {
                        Some(i)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .map(|id| *self.values.get(&id).unwrap())
            .collect()
    }

    pub fn value(&self, id: CellID) -> Option<T> {
        self.values.get(&id).and_then(|value| Some(*value))
    }

    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if self.cells.iter().any(|(i, _)| *i == CellID::Input(id)) {
            let old_values = self.values.clone();
            self.set_cell_value_recursively(CellID::Input(id), new_value);
            self.values.iter().for_each(|(id, new)| {
                if old_values.get(id) != Some(new) {
                    self.callbacks
                        .iter()
                        .for_each(|(cell_id, _, callback, enabled)| {
                            if *cell_id == *id && *enabled {
                                callback.borrow_mut().deref_mut()(*new);
                            }
                        });
                }
            });
            true
        } else {
            false
        }
    }

    fn set_cell_value_recursively(&mut self, id: CellID, new_value: T) {
        if let Some(value) = self.values.get(&id) {
            if *value == new_value {
                return;
            }
        }

        self.values.insert(id, new_value);

        self.get_outputs(id).iter().for_each(|cid| {
            let dependencies = self.get_dependencies_values(*cid);
            let output_value = self.funcs.get(cid).unwrap()(dependencies.as_slice());
            self.set_cell_value_recursively(*cid, output_value);
        });
    }

    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        let callback_id = CallbackID(self.new_callback_id_inner());
        let cell_id = CellID::Compute(id);
        self.cells
            .iter()
            .any(|(_, o)| *o == Some(cell_id))
            .then(|| {
                self.callbacks
                    .push((cell_id, callback_id, Box::new(RefCell::new(callback)), true));
                callback_id
            })
    }

    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let cell_id = CellID::Compute(cell);
        if !self.cells.iter().any(|(_, o)| *o == Some(cell_id)) {
            return Err(RemoveCallbackError::NonexistentCell);
        }

        let mut find = false;
        self.callbacks
            .iter_mut()
            .for_each(|(_, callback_id, _, enabled)| {
                if *callback_id == callback && *enabled {
                    *enabled = false;
                    find = true;
                }
            });

        if !find {
            Err(RemoveCallbackError::NonexistentCallback)
        } else {
            Ok(())
        }
    }
}
