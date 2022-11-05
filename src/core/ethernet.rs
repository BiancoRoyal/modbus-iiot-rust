use core::modbusreturn::{ModbusReturnCoils, ModbusReturnRegisters};

/// MODBUS API (based on the technical protocol specification)
pub trait EthernetMaster {
    ///	MODBUS Function Code 1 (0x01)
    /// This function code is used to read from 1 to 2000 contiguous
    /// status of coils in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.read_coils(0, 16);
    ///     println!("Response CO: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn read_coils(&mut self, starting_address: u16, quantity_of_coils: u16) -> ModbusReturnCoils;

    ///	MODBUS Function Code 2 (0x02)
    /// This function code is used to read from 1 to 2000 contiguous
    /// status of discrete inputs in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.read_discrete_inputs(0, 1);
    ///     println!("Response of read_discrete_inputs: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn read_discrete_inputs(
        &mut self,
        starting_address: u16,
        quantity_of_inputs: u16,
    ) -> ModbusReturnCoils;

    ///	MODBUS Function Code 3 (0x03)
    /// This function code is used to read the contents of a contiguous
    /// block of holding registers in a remote  device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.read_holding_registers(0, 32);
    ///     println!("Response of read_holding_registers: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn read_holding_registers(
        &mut self,
        starting_address: u16,
        quantity_of_registers: u16,
    ) -> ModbusReturnRegisters;

    ///	MODBUS Function Code 4 (0x04)
    /// This function code is used to read from 1 to 125 contiguous
    /// input registers in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.read_input_registers(0, 10);
    ///     println!("Response of read_input_registers: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn read_input_registers(
        &mut self,
        starting_address: u16,
        quantity_of_input_registers: u16,
    ) -> ModbusReturnRegisters;

    /// MODBUS Function Code 5 (0x05)
    /// This function code is used to write a single output to
    /// either ON or OFF in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     //  set to true
    ///     let response = client.write_single_coil(0, 0xFF00);
    ///     println!("Response of write_single_coil: {:?}", response);
    ///
    ///     //  set to false
    ///     let response = client.write_single_coil(0, 0x0000);
    ///     println!("Response of write_single_coil: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn write_single_coil(&mut self, output_address: u16, output_value: u16) -> ModbusReturnCoils;

    ///	MODBUS Function Code 6 (0x06)
    /// This function code is used to write a single
    /// holding register in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.write_single_register(0, 42);
    ///     println!("Response of write_single_register: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn write_single_register(
        &mut self,
        register_address: u16,
        register_value: u16,
    ) -> ModbusReturnRegisters;

    ///	MODBUS Function Code 15 (0x0F)
    /// This function code is used to force each coil in a
    /// sequence of coils to either ON or OFF in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.write_multiple_coils(0, 10, vec![ 0xCD, 0x01 ]);
    ///     println!("Response of write_multiple_coils: {:?}", response);
    ///
    ///     client.disconnect();
    /// }
    /// ```
    fn write_multiple_coils(
        &mut self,
        starting_address: u16,
        quantity_of_outputs: u16,
        outputs_value: Vec<u8>,
    ) -> ModbusReturnRegisters;

    ///	MODBUS Function Code 16 (0x10)
    /// This function code is used to write a block of contiguous
    /// registers (1 to 123 registers) in a remote device.
    /// (Description from the MODBUS APPLICATION PROTOCOL SPECIFICATION V1.1b3)
    ///
    /// # Example
    ///
    /// ```rust,no_run
    ///
    /// use modbus_iiot::tcp::master::*;
    /// use modbus_iiot::core::ethernet::EthernetMaster;
    ///
    /// let mut client = TcpClient::new("127.0.0.1");
    ///
    /// if let Err(message) = client.connect()
    /// {
    ///     println!("failure = {}", message);
    /// }
    /// else
    /// {
    ///     let response = client.write_multiple_registers(0, vec![ 12345, 1, 6789, 42 ]);
    ///     println!("Response of write_multiple_registers: {:?}", response);
    ///    
    ///     client.disconnect ();    
    /// }
    /// ```
    fn write_multiple_registers(
        &mut self,
        starting_address: u16,
        register_values: Vec<u16>,
    ) -> ModbusReturnRegisters;
}
