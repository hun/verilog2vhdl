library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity mux3d is
    generic (
        N : integer := 8;
        M : integer := 4
    );
    port (
        sel: in std_logic_vector(N*M-1 downto 0);
        in_a: in std_logic_vector(N-1 downto 0);
        in_b: in std_logic_vector(N-1 downto 0);
        out: out std_logic_vector(N*M-1 downto 0)
    );
end entity mux3d;

architecture rtl of mux3d is
begin
    -- Internal logic stub
end architecture rtl;

