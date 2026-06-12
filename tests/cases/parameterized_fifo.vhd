library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity fifo is
    generic (
        DATA_WIDTH : integer := 8;
        ADDR_SIZE : integer := 4
    );
    port (
        clk: in std_logic;
        rst: in std_logic;
        wr_en: in std_logic;
        rd_en: in std_logic;
        data_in: in std_logic_vector(DATA_WIDTH-1 downto 0);
        data_out: out std_logic_vector(DATA_WIDTH-1 downto 0);
        count: out std_logic_vector(ADDR_SIZE-1 downto 0)
    );
end entity fifo;

architecture rtl of fifo is
begin
    -- Internal logic stub
end architecture rtl;

