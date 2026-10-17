<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;

// getRouteKeyName forms that P2R-480 does not admit (P2R-482): an #[\Override] that
// overrides nothing here, another modifier, a parameter, a nullable type, a body that is
// no literal, an escape PHP refuses, and a return by reference.
final class OverridesNothing extends Model
{
    protected $table = 'items';

    #[\Override]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

final class Hidden extends Model
{
    protected $table = 'items';

    protected function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

final class Takes extends Model
{
    protected $table = 'items';

    public function getRouteKeyName(string $prefix): string
    {
        return 'uuid';
    }
}

final class Maybe extends Model
{
    protected $table = 'items';

    public function getRouteKeyName(): ?string
    {
        return 'uuid';
    }
}

final class Computed extends Model
{
    protected $table = 'items';

    public function getRouteKeyName(): string
    {
        return strtolower('UUID');
    }
}

final class Escaped extends Model
{
    protected $table = 'items';

    public function getRouteKeyName(): string
    {
        return "\u{zz}";
    }
}

final class Referenced extends Model
{
    protected $table = 'items';

    public function &getRouteKeyName(): string
    {
        return 'uuid';
    }
}

// Another attribute keeps P2R-401's rule, which takes the struct away.
final class Marked extends Model
{
    protected $table = 'items';

    #[\Deprecated]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

// Links that P2R-481 does not admit, each named in its stop: another method, another
// letter case, a pivot link on another type, a constant, a negative number, no argument,
// an escape PHP refuses, the first of two, an unpacked argument; the helper's own
// condition 10 comes first.
final class Links extends Model
{
    protected $table = 'items';

    public function newest(): HasMany
    {
        return $this->hasMany(Links::class)->latest();
    }

    public function stamped(): BelongsToMany
    {
        return $this->belongsToMany(Links::class)->WithTimestamps();
    }

    public function pivotOnMany(): HasMany
    {
        return $this->hasMany(Links::class)->withTimestamps();
    }

    public function constant(): BelongsToMany
    {
        return $this->belongsToMany(Links::class)->withPivot(PHP_EOL);
    }

    public function negative(): HasMany
    {
        return $this->hasMany(Links::class)->limit(-1);
    }

    public function none(): HasMany
    {
        return $this->hasMany(Links::class)->with();
    }

    public function escaped(): BelongsToMany
    {
        return $this->belongsToMany(Links::class)->withPivot("\u{zz}");
    }

    public function twoBad(): HasMany
    {
        return $this->hasMany(Links::class)->latest()->oldest();
    }

    public function unpacked(): HasMany
    {
        return $this->hasMany(Links::class)->with(...['author']);
    }

    public function helperFirst(): HasMany
    {
        return $this->hasmany(Links::class)->latest();
    }
}
