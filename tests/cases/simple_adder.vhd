library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity simple_adder is
    port (
    -- //Clock signal
        clk: in std_logic;
    -- //Reset signal (active low)
        rst_n: in std_logic;
    -- //Input A
        a: in std_logic_vector(7 downto 0);
    -- //Input B
        b: in std_logic_vector(7 downto 0);
    -- //Sum output
        sum: out std_logic_vector(7 downto 0)
    );
end entity simple_adder;

architecture rtl of simple_adder is
begin
    -- Internal logic stub
end architecture rtl;

