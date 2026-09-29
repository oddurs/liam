/** Target boot trace on Firecracker. A design budget, not a measurement. */
export interface TraceLine {
  /** Simulated milliseconds since VMM start. */
  t: number;
  source: string;
  message: string;
  final?: boolean;
}

export const bootTrace: TraceLine[] = [
  { t: 0.0, source: 'vmm', message: 'PVH direct boot, no firmware, no bootloader' },
  { t: 3.812, source: 'kernel', message: 'Linux LTS, uncompressed, initramfs built in' },
  { t: 8.644, source: 'kernel', message: 'virtio-mmio net0 up, no PCI scan' },
  { t: 10.921, source: 'kernel', message: 'run /init' },
  { t: 11.73, source: 'init', message: 'root mounted read-only, sysctls applied' },
  { t: 12.905, source: 'init', message: 'bound :80 :443, SO_REUSEPORT x 2 cores' },
  { t: 14.188, source: 'liamd', message: 'site pack mapped, 1,284 files, 38 MB' },
  { t: 15.52, source: 'liamd', message: 'io_uring rings up, kTLS on' },
  { t: 16.307, source: 'liamd', message: 'ready, serving acme.com' },
  { t: 19.044, source: 'http', message: 'GET / 200, first byte', final: true },
];
