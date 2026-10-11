<?php

declare(strict_types=1);

// rustc 1.99 accepts `top += 1` here only because print! borrows top. php2rust stops on
// every overflow it can prove (P2R-50).
$top = 9223372036854775807;
$top += 1;
echo $top, "\n";
