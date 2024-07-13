// PL011 UART

use register::register_bitfields;

register_bitfields! {
    u32,
    // Data register
    DR [
        DATA OFFSET(0) NUMBITS(8) []
        // Framing error
        FE OFFSET(8) NUMBITS(1) [],
        // Parity error
        PE OFFSET(9) NUMBITS(1) [],
        // Break error
        BE OFFSET(10) NUMBITS(1) [],
        // Overrun error
        OE OFFSET(11) NUMBITS(1) [],
    ],
    // Flag register
    FR [
        // Clear to send
        CTS OFFSET(0) NUMBITS(1) [],
        // Data set ready
        DSR OFFSET(1) NUMBITS(1) [],
        // Data carrier detect
        DCD OFFSET(2) NUMBITS(1) [],
        BUSY OFFSET(3) NUMBITS(1) [],
        // Receive FIFO empty
        RXFE OFFSET(4) NUMBITS(1) [],
        // Transmit FIFO full
        TXFF OFFSET(5) NUMBITS(1) [],
        // Receive FIFO full
        RXFF OFFSET(6) NUMBITS(1) [],
        // Transmit FIFO empty
        TXFE OFFSET(7) NUMBITS(1) [],
        // Ring indicator
        RI OFFSET(8) NUMBITS(1) [],
    ],
    // Low power counter register
    ILPR [
        ILPDVSR OFFSET(0) NUMBITS(8) [],
    ],
    // Integer baud rate register
    IBRD [
        DIVINT OFFSET(0) NUMBITS(16) [],
    ],
    // Fractional baud rate register
    FBRD [
        DIVFRAC OFFSET(0) NUMBITS(6) [],
    ],
    // Line Control register
    LCR_H [
        // Send break
        BRK OFFSET(0) NUMBITS(1) [],
        // Parity enable
        PEN OFFSET(1) NUMBITS(1) [],
        // Even parity select
        EPS OFFSET(2) NUMBITS(1) [],
        // Two stop bits select
        STP2 OFFSET(3) NUMBITS(1) [],
        // Enable FIFOs
        FEN OFFSET(4) NUMBITS(1) [],
        // Word length
        WLEN OFFSET(5) NUMBITS(2) [
            FiveBits = 0b00,
            SixBits = 0b01,
            SevenBits = 0b10,
            EightBits = 0b11,
        ],
        // Stick parity
        SPS OFFSET(7) NUMBITS(1) [],
    ],
    // Control register
    CR [
        // Uart enable
        UARTEN OFFSET(0) NUMBITS(1) [],
        // SIR enable
        SIREN OFFSET(1) NUMBITS(1) [],
        // SIR low power mode
        SIRLP OFFSET(2) NUMBITS(1) [],
        // Loopback enable
        LBE OFFSET(7) NUMBITS(1) [],
        // Transmit enable
        TXE OFFSET(8) NUMBITS(1) [],
        // Receive enable
        RXE OFFSET(9) NUMBITS(1) [],
        // Data transmit ready
        DTR OFFSET(10) NUMBITS(1) [],
        // Request to send
        RTS OFFSET(11) NUMBITS(1) [],
        OUT1 OFFSET(12) NUMBITS(1) [],
        OUT2 OFFSET(13) NUMBITS(1) [],
        // RTS hardware flow control enable
        RTSEN OFFSET(14) NUMBITS(1) [],
        // CTS hardware flow control enable
        CTSEN OFFSET(15) NUMBITS(1) [],
    ],
    // Interrupt FIFO level register
    IFLS [
        // Transmit interrupt FIFO level select
        TXIFLSEL OFFSET(0) NUMBITS(3) [
            OneEigth = 0b000,
            OneFourth = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEigths = 0b100,
        ],
        // Receive interrupt FIFO level select
        RXIFLSEL OFFSET(3) NUMBITS(3) [
            OneEigth = 0b000,
            OneFourth = 0b001,
            OneHalf = 0b010,
            ThreeQuarters = 0b011,
            SevenEigths = 0b100,
        ],
    ],
    // Interrupt mask register
    IMSC [
        // UARTRIINTR mask
        RIMIM OFFSET(0) NUMBITS(1) [],
        // UARTCTSINTR mask
        CTSMIM OFFSET(1) NUMBITS(1) [],
        // UARTDCDINTR mask
        DCDMIM OFFSET(2) NUMBITS(1) [],
        // UARTDSRINTR mask
        DSRMIM OFFSET(3) NUMBITS(1) [],
        // UARTRXINTR mask
        RXIM OFFSET(4) NUMBITS(1) [],
        // UARTTXINTR mask
        TXIM OFFSET(5) NUMBITS(1) [],
        // UARTRTINTR mask
        RTIM OFFSET(6) NUMBITS(1) [],
        // UARTFEINTR mask
        FEIM OFFSET(7) NUMBITS(1) [],
        // UARTPEINTR mask
        PEIM OFFSET(8) NUMBITS(1) [],
        // UARTBEINTR mask
        BEIM OFFSET(9) NUMBITS(1) [],
        // UARTOEINTR mask
        OEIM OFFSET(10) NUMBITS(1) [],
    ],
    // Raw interrupt status register
    RIS [
        RIRMIS OFFSET(0) NUMBITS(1) [],
        CTSRMIS OFFSET(1) NUMBITS(1) [],
        DCDRMIS OFFSET(2) NUMBITS(1) [],
        DSRRMIS OFFSET(3) NUMBITS(1) [],
        RXRIS OFFSET(4) NUMBITS(1) [],
        TXRIS OFFSET(5) NUMBITS(1) [],
        RTRIS OFFSET(6) NUMBITS(1) [],
        FERIS OFFSET(7) NUMBITS(1) [],
        PERIS OFFSET(8) NUMBITS(1) [],
        BERIS OFFSET(9) NUMBITS(1) [],
        OERIS OFFSET(10) NUMBITS(1) [],
    ],
    // Masked interrupt status register
    MIS [
        RIMMIS OFFSET(0) NUMBITS(1) [],
        CTSMMIS OFFSET(1) NUMBITS(1) [],
        DCDMMIS OFFSET(2) NUMBITS(1) [],
        DSRMMIS OFFSET(3) NUMBITS(1) [],
        RXMIS OFFSET(4) NUMBITS(1) [],
        TXMIS OFFSET(5) NUMBITS(1) [],
        RTMIS OFFSET(6) NUMBITS(1) [],
        FEMIS OFFSET(7) NUMBITS(1) [],
        PEMIS OFFSET(8) NUMBITS(1) [],
        BEMIS OFFSET(9) NUMBITS(1) [],
        OEMIS OFFSET(10) NUMBITS(1) [],
    ],
    // Interrupt clear register
    ICR [
        // Clears the UARTRIINTR interrupt
        RIMIC OFFSET(0) NUMBITS(1) [],
        // Clears the UARTCTSINTR interrupt
        CTSMIC OFFSET(1) NUMBITS(1) [],
        // Clears the UARTDCDINTR interrupt
        DCDMIC OFFSET(2) NUMBITS(1) [],
        // Clears the UARTDSRINTR interrupt
        DSRMIC OFFSET(3) NUMBITS(1) [],
        // Clears the UARTRXINTR interrupt
        RXIC OFFSET(4) NUMBITS(1) [],
        // Clears the UARTTXINTR interrupt
        TXIC OFFSET(5) NUMBITS(1) [],
        // Clears the UARTRTINTR interrupt
        RTIC OFFSET(6) NUMBITS(1) [],
        // Clears the UARTFEINTR interrupt
        FEIC OFFSET(7) NUMBITS(1) [],
        // Clears the UARTPEINTR interrupt
        PEIC OFFSET(8) NUMBITS(1) [],
        // Clears the UARTBEINTR interrupt
        BEIC OFFSET(9) NUMBITS(1) [],
        // Clears the UARTOEINTR interrupt
        OEIC OFFSET(10) NUMBITS(1) [],
    ],
    // DMA control register
    DMACR [
        RXDMAE OFFSET(0) NUMBITS(1) [],
        RXDMAE OFFSET(1) NUMBITS(1) [],
        DMAONERR OFFSET(2) NUMBITS(2) [],
    ],
}
#[repr(C, packed)]
pub struct PL011Uart {
    uart_dr: u16,           // 0x000
    _reserved0: [u8; 2],    // 0x002
    uart_rsr_ecr: u8,       // 0x004
    _reserved1: [u8; 19],   // 0x005
    uart_fr: u16,           // 0x018
    _reserved2: [u8; 6],    // 0x01a
    uart_ilpr: u8,          // 0x020
    _reserved3: [u8; 1], 
    uart_ibrd: u16,
    uart_fbrd: u8,
    uart_lcr_h: u8,
    uart_cr: u16,
    uart_ifls: u8,
    uart_imsc: u16,
    uart_ris: u16,
    uart_mis: u16,
    uart_ict: u16,
    uart_dmacr: u8
}