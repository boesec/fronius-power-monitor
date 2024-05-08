# Fronius Power Monitor

fronius-power-monitor is a convenient tool for monitoring Fronius inverter-based photovoltaic (PV) systems. It extracts crucial performance data and stores it in InfluxDB, facilitating the creation of insightful dashboards for visualizing power generation, storage patterns, and overall system efficiency

## Technologies Used

* Rust: A modern systems programming language focused on performance, reliability, and safety.
* Fronius REST API: Provides the interface to communicate with and collect data from your Fronius inverter system.
* InfluxDB: A time-series database optimized for storing and visualizing performance metrics like those from your PV system. 


## Installation

**Prerequisites:**

* Fronius PV System: An active Fronius inverter-based PV system accessible over your network.
* InfluxDB v2.x: A running InfluxDB v2.x instance with permissions to create a database (bucket) and write data.

**Instructions:**

1. Locate the config Directory: The configuration file needs to be placed  in a directory named "config", located next to the fronius-power-monitor executable.
2. Create fronius-power-monitor.config:  Within the "config" directory, create a file named fronius-power-monitor.config.
3. Replace [the example config](https://github.com/boesec/fronius-power-monitor/blob/main/systemd/fronius-power-monitor.config) with the actual data for your setup.

## Usage

* Prerequisites: Ensure you have a correctly configured fronius-power-monitor.config file in the "config" directory as detailed in the Installation section.

* Running the Tool: Simply execute the fronius-power-monitor binary. Once started, it will continuously collect data from your Fronius inverter and store it in your InfluxDB database.

* Automation (Optional): For automated start/stop control, you can create a systemd service (on Linux systems) or a similar service mechanism for Windows or macOS.

## Contributing

1. Fork it (<https://github.com/yourname/yourproject/fork>)
2. Create your feature branch (`git checkout -b feature/fooBar`)
3. Commit your changes (`git commit -am 'Add some fooBar'`)
4. Push to the branch (`git push origin feature/fooBar`)
5. Create a new Pull Request

## License

This project is licensed under the GNU General Public License v3.0 (GPLv3). For the full license text, please see [LICENSE.md](link to your LICENSE.md file) or visit https://www.gnu.org/licenses/gpl-3.0.en.html

## Release History

* 0.1.0
    * The first proper release
