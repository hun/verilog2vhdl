library ieee;
use ieee.std_logic_1164.all;
use ieee.numeric_std.all;

entity signed_unsigned is
    generic (
        WIDTH : integer := 16
    );
    port (
        -- Signed input
        a: in signed(WIDTH-1 downto 0);
        -- Unsigned input
        b: in unsigned(WIDTH-1 downto 0);
        -- Signed result
        sum: out signed(WIDTH-1 downto 0);
        -- Unsigned result
        product: out unsigned(WIDTH-1 downto 0)
    );
end entity signed_unsigned;

architecture rtl of signed_unsigned is
begin
    -- Internal logic stub
end architecture rtl;

