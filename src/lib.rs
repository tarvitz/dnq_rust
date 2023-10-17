use std::{thread, sync::mpsc, option::Option};
use std::sync::{Arc, Mutex};
use log::{info};

enum Message {
	NewJob(Job),
	Terminate,
}

pub struct ThreadPool {
	workers: Vec<Worker>,
	sender: mpsc::Sender<Message>,
}

trait FnBox {
	fn call_box(self: Box<Self>);
}

impl <F: FnOnce()> FnBox for F {
	fn call_box(self: Box<F>){
		(*self)()
	}
}

type Job = Box<dyn FnBox + Send + 'static>;

impl ThreadPool {
	/// Create new thread pool.
	///
	/// size -- is an amount of workers in the pool.
	///
	/// # Panics
	/// 1. `new` raises a panic, if size is equal to zero ([`assert_ne!`]);
	///
	/// TODO: use Result<ThreadPool, ThreadPoolCreatingError>.
	pub fn new(size: usize) -> ThreadPool {
		assert_ne!(0, size);

		let mut workers = Vec::with_capacity(size);
		let (sender, receiver) = mpsc::channel();
		let receiver = Arc::new(Mutex::new(receiver));

		for id in 0..size {
			workers.push(Worker::new(id, Arc::clone(&receiver)));
		}

		ThreadPool { workers, sender }
	}

	pub fn execute<F>(&self, f: F)
		where F: FnOnce() + Send + 'static {

		let job = Box::new(f);
		self.sender.send(Message::NewJob(job)).unwrap();
	}
}

impl Drop for ThreadPool {
	fn drop(&mut self) {
		info!("all workers sent message abouts their termination.");

		for _ in &mut self.workers {
			self.sender.send(Message::Terminate).unwrap();
		}

		info!("All workers are setting down.");

		for worker in &mut self.workers {
			info!("Unregistering worker {}", worker.id);
			if let Some(thread) = worker.thread.take(){
				thread.join().unwrap();
			}
		}
	}
}

struct Worker {
	id: usize,
	thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
	fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
		let thread = thread::spawn(move || {
			loop {
				let message = receiver.lock().unwrap().recv().unwrap();

				match message {
					Message::NewJob(job) => {
						info!("Worker {} received a task, executing.", id);
						job.call_box();
					},
					Message::Terminate => {
						info!("Worker {} should be terminated.", id);
						break;
					},
				}
			}
		});

		Worker{ id, thread: Some(thread) }
	}
}
