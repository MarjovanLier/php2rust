<?php

namespace App;

// In namespace App an unqualified helper() calls this function, not the global one.
function helper(): int
{
    return 2;
}
