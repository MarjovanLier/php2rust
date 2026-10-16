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

// PHP rejects a second constructor, in any letter case, and a modifier written twice.
class TwoConstructors extends \Exception
{
    public function __construct(string $m)
    {
        parent::__construct($m);
    }

    public function __CONSTRUCT(string $m)
    {
        parent::__construct($m);
    }
}

final final class Doubled extends \Exception
{
    public public function __construct(public public string $what = 'x')
    {
        parent::__construct($what);
    }
}

// A call that throws in the arguments of the parent call.
class Described extends \Exception
{
    public function __construct(int $n)
    {
        parent::__construct(describe($n));
    }
}

function describe(int $n): string
{
    if ($n < 0) {
        throw new \InvalidArgumentException('negative');
    }
    return 'n=' . $n;
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

// More forms the function cannot hold.
class ByReference extends \Exception
{
    public function &__construct(string $m)
    {
        parent::__construct($m);
    }
}

class ThirdArgument extends \Exception
{
    public function __construct(string $m)
    {
        parent::__construct($m, 0, null);
    }
}

class PromotedCode extends \Exception
{
    public function __construct(public int $code = 1)
    {
        parent::__construct('c', $code);
    }
}

class StaticPromoted extends \Exception
{
    public function __construct(public static string $what = 'x')
    {
        parent::__construct($what);
    }
}

class EndlessLoop extends \Exception
{
    public function __construct(string $m)
    {
        while (true) {
            echo "spin\n";
        }
        parent::__construct($m);
    }
}

class AbstractPromoted extends \Exception
{
    public function __construct(abstract string $what = 'x', private public(set) string $who = 'y')
    {
        parent::__construct($what . $who);
    }
}

final class Item
{
    public function __construct(public int $qty) {}
}

class ChangesItem extends \Exception
{
    public function __construct(Item $item)
    {
        $item->qty = 0;
        parent::__construct('changed');
    }
}

class ThrowsInBody extends \Exception
{
    public function __construct(string $m)
    {
        if ($m === '') {
            throw new \LogicException('empty');
        }
        parent::__construct($m);
    }
}

$shadowed = 'x';
echo $shadowed, "\n";
