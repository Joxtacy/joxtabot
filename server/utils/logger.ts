enum LogLevel {
  DEBUG = "debug",
  ERROR = "error",
  INFO = "info",
  WARN = "warn",
}

export class Logger {
  private logging: boolean;

  constructor() {
    this.logging = Deno.env.get("JOXTABOT_DEBUG") !== "";
  }

  private log(logLevel: LogLevel, ...data: any[]): void {
    console[logLevel](...data);
  }

  public debug(...data: any[]): void {
    this.log(LogLevel.DEBUG, ...data);
  }

  public error(...data: any[]): void {
    this.log(LogLevel.ERROR, ...data);
  }

  public info(...data: any[]): void {
    this.log(LogLevel.INFO, ...data);
  }

  public warn(...data: any[]): void {
    this.log(LogLevel.WARN, ...data);
  }
}
