struct MacOSService {
    name: str
}

trait Service {
    fn running() -> bool;
}

impl Service for MacOSService {
    fn running() -> bool {}
}