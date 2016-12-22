
use std::thread;
use std::sync;
use std::sync::mpsc;
use std::mem;
use std::fmt;
use std::error::Error;
use std::any::Any;

use num_complex::{Complex};
use fractal::{Fractal, FractalOrbit};
use grid;

#[derive(Debug)]
pub enum RunnerError {
    RunnerError(String),
    SendError(Box<Error + Send + 'static>),
    RecvError(mpsc::RecvError),
    ThreadError(Box<Any + Send + 'static>),
}
type RunnerResult<T> = Result<T, RunnerError>;

pub trait FractalRunner {
    fn run(&self, grid: &grid::Grid) -> RunnerResult<Vec<FractalOrbit>>;
}

pub struct SyncronousRunner<T: Fractal> {
    fractal: T,
}

pub struct MultiThreadedRunner<T> {
    fractal: T,
    num_threads: usize,
}

impl fmt::Display for RunnerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RunnerError::SendError(ref err) => err.fmt(f),
            RunnerError::RecvError(ref err) => err.fmt(f),
            RunnerError::RunnerError(ref msg) => 
                write!(f, "{}", msg),
            RunnerError::ThreadError(_) => write!(f, "thread error"),
        }
    }
}

impl Error for RunnerError {
    fn description(&self) -> &str {
        match *self {
            RunnerError::SendError(ref err) => err.description(),
            RunnerError::RecvError(ref err) => err.description(),
            RunnerError::RunnerError(ref msg) =>
                &msg,
            RunnerError::ThreadError(_) => "thread error",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            RunnerError::SendError(ref err) => Some(&**err),
            RunnerError::RecvError(ref err) => Some(err),
            _ => None
        }
    }
}

impl From<mpsc::RecvError> for RunnerError {
    fn from(other: mpsc::RecvError) -> Self {
        RunnerError::RecvError(other)
    }
}

impl<T: Send + 'static> From<mpsc::SendError<T>> for RunnerError {
    fn from(other: mpsc::SendError<T>) -> Self {
        RunnerError::SendError(Box::new(other))
    }
}

impl From<Box<Any + Send + 'static>> for RunnerError {
    fn from(other: Box<Any + Send + 'static>) -> Self {
        RunnerError::ThreadError(other)
    }
}

impl<T: Fractal> SyncronousRunner<T> {
    pub fn new(fractal: T) -> SyncronousRunner<T> {
        SyncronousRunner::<T>{fractal: fractal}
    }
}

impl<'a, T: Fractal> SyncronousRunner<T> {
    pub fn fractal(&'a self) -> &'a T {
        &self.fractal
    }
}

impl<T: Fractal> FractalRunner for SyncronousRunner<T> {
    fn run(&self, grid: &grid::Grid) -> RunnerResult<Vec<FractalOrbit>> {
        let values = grid.iter()
            .map(|(x,y)| self.fractal.test(Complex::new(x, y)))
            .collect();

        Ok(values)
    }
}

impl<T: Fractal + Send + Sync + 'static> MultiThreadedRunner<T> {
    pub fn new(fractal: T, num_threads: usize) -> Self {
        MultiThreadedRunner{fractal: fractal, num_threads: num_threads} 
    }

    fn execute_workers(&self, grid: &grid::Grid) -> RunnerResult<Vec<FractalOrbit>> {
        const STRIP_HEIGHT: u32 = 1; 

        let mut values = Vec::with_capacity(grid.num_cells());
        values.resize(grid.num_cells(), FractalOrbit::Bounded);

        let fractal = sync::Arc::new(self.fractal.clone());

        let mut threads = Vec::with_capacity(self.num_threads);
        let mut senders = Vec::with_capacity(self.num_threads);

        let (row_sender, row_receiver) = 
            mpsc::channel::<(grid::GridStrip, Vec<FractalOrbit>)>();

        for _ in 0..self.num_threads {
            let (tx, rx) = mpsc::channel::<Option<grid::GridStrip>>();
            let grid_copy = grid.clone();
            let fractal = fractal.clone();
            let row_sender = row_sender.clone();

            let thread = thread::spawn(move || {
                return thread_worker(fractal, grid_copy, rx, row_sender);
            });

            threads.push(Some(thread)); 
            senders.push(tx);
        }
        mem::drop(row_sender);

        let mut thread_target = 0;
        for strip in grid.iter_strips(STRIP_HEIGHT) {
            try!(senders[thread_target].send(Some(strip)));
            thread_target += 1;
            if thread_target == self.num_threads {
                thread_target = 0;
            }
        }

        for sender in senders {
            try!(sender.send(None));
        }

        for (strip, data) in row_receiver {
            let strip_start = grid.row_start(strip.start);
            let strip_end = strip_start + (strip.height*grid.cells_wide()) as usize;

            values[strip_start..strip_end].copy_from_slice(&data);
        }

        for worker in threads.iter_mut() {
            let ret = try!(worker.take().unwrap().join());

            if let Err(e) = ret {
                return Err(e)
            }
        }

        Ok(values)
    }
}

fn thread_worker<T: Fractal + Send + Sync + 'static>(fractal: sync::Arc<T>, 
        grid: grid::Grid, recver: mpsc::Receiver<Option<grid::GridStrip>>,
        row_sender: mpsc::Sender<(grid::GridStrip, Vec<FractalOrbit>)>) -> RunnerResult<()>{
    loop {
        let item = try!(recver.recv());
        match item {
            Some(strip) => {
                let escape_times: Vec<_> = strip.iter(&grid).map(|(x,y)| {
                        fractal.test(Complex::new(x, y))
                    }).collect();

                try!(row_sender.send((strip.clone(), escape_times)));
            },
            None => break,
        }
    }
    Ok(())
}

impl<T: Fractal + Send + Sync + 'static> FractalRunner for MultiThreadedRunner<T> {
    fn run(&self, grid: &grid::Grid) -> RunnerResult<Vec<FractalOrbit>> {
        let values = self.execute_workers(grid);

        values
    }
}
