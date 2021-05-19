use tokio::sync::oneshot::Receiver;

struct Arm9Bus {
    arm9: Receiver<()>,
}
