library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity pre_post_content is
    port (
        -- Clock signal
        clk: in std_logic;
        -- Reset signal (active low)
        rst_n: in std_logic;
        -- Input data
        data_in: in std_logic_vector(7 downto 0);
        -- Output data
        data_out: out std_logic_vector(7 downto 0)
    );
end entity pre_post_content;

architecture rtl of pre_post_content is
begin
    -- Internal logic stub
end architecture rtl;

