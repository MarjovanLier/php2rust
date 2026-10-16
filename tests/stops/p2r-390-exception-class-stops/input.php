<?php

declare(strict_types=1);

namespace App\Exceptions;

// An exception class may hold a constructor and nothing else.
class WithMethod extends \Exception
{
    public function detail(): string
    {
        return 'detail';
    }
}

class WithProperty extends \Exception
{
    public string $field = '';
}

#[\AllowDynamicProperties]
class WithAttribute extends \Exception {}

abstract class Abstracted extends \Exception {}

class WithInterface extends \Exception implements \Stringable
{
    public function __toString(): string
    {
        return 'x';
    }
}

final class Sealed extends \Exception {}

// PHP rejects a class that extends a final class.
class AfterSealed extends Sealed {}

// PHP declares a child before its parent only when execution reaches it.
class Early extends Late {}

class Late extends \Exception {}

// A use other than throw new, catch, extends and use stops.
function make(): Sealed
{
    return new Sealed('made');
}

$made = new Late('made');
echo Late::class, "\n";
