<?php

// A small stand-in for Eloquent: the parts these tests name. It is not the framework. Its
// configuration properties carry Laravel 12.35.1's docblock types, against which Mago
// checks a model's own.

namespace Illuminate\Database\Eloquent\Concerns;

trait GuardsAttributes
{
    /**
     * @var array<int, string>
     */
    protected $fillable = [];
}

trait HasAttributes
{
    /**
     * @var array
     */
    protected $casts = [];

    /**
     * @var array
     */
    protected $appends = [];
}

// Unlike Laravel's, this trait gives $hidden a native type, which a model must repeat.
trait HidesAttributes
{
    protected array $hidden = [];
}

namespace Illuminate\Database\Eloquent;

use Illuminate\Database\Eloquent\Concerns\GuardsAttributes;
use Illuminate\Database\Eloquent\Concerns\HasAttributes;
use Illuminate\Database\Eloquent\Concerns\HidesAttributes;

abstract class Model
{
    use GuardsAttributes;
    use HasAttributes;
    use HidesAttributes;

    /** @var array<string, mixed> */
    private array $attributes = [];

    public function __get(string $key): mixed
    {
        return $this->attributes[$key] ?? null;
    }
}
