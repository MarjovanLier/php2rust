<?php

declare(strict_types=1);

namespace App\Exceptions;

// PHP rejects these constructors, or warns about them, and Mago reports nothing.
class StaticConstructor extends \Exception
{
    public static function __construct(string $m)
    {
        parent::__construct($m);
    }
}

class TypedConstructor extends \Exception
{
    public function __construct(string $m): void
    {
        parent::__construct($m);
    }
}

class HiddenConstructor extends \Exception
{
    private function __construct(string $m)
    {
        parent::__construct($m);
    }
}

// A hook runs when the constructor sets the property.
class HookedProperty extends \Exception
{
    public function __construct(
        public string $entity { set { $this->entity = $value; } },
    ) {
        parent::__construct($entity);
    }
}

// The body must end with its one parent::__construct call.
class NoParentCall extends \Exception
{
    public function __construct(string $m)
    {
        $copy = $m;
    }
}

class ThisRead extends \Exception
{
    public function __construct(public readonly string $entity)
    {
        parent::__construct($this->entity);
    }
}

class EarlyReturn extends \Exception
{
    public function __construct(string $m)
    {
        if ($m === '') {
            return;
        }
        parent::__construct($m);
    }
}

// A promoted property PHP's Exception already declares.
class PromotedMessage extends \Exception
{
    public function __construct(public $message = 'm')
    {
        parent::__construct($message);
    }
}

// PHP would run the parent's constructor.
class Base extends \Exception
{
    public function __construct(string $entity)
    {
        parent::__construct($entity . ' failed');
    }
}

class Child extends Base {}

// The Rust name of another function.
class Check extends \Exception
{
    public function __construct(string $m)
    {
        parent::__construct($m);
    }
}

function check(): string
{
    return 'x';
}

// A variable or parameter of the function's Rust name would hide it (P2R-31).
class Shadowed extends \Exception
{
    public function __construct(string $m = 'shadowed')
    {
        parent::__construct($m);
    }
}

function hide(string $shadowed): string
{
    return $shadowed;
}

$shadowed = 'x';
echo $shadowed, "\n";
