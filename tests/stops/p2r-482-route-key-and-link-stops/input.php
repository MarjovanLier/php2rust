<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Factories\HasFactory;
use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\Relations\BelongsToMany;
use Illuminate\Database\Eloquent\Relations\HasMany;

// getRouteKeyName forms that P2R-480 does not admit (P2R-482): an #[\Override] while
// Model takes the method only from a trait, another modifier, a parameter, a nullable
// type, a body that is no literal, an escape PHP refuses, and a return by reference.
final class OverridesNothing extends Model
{
    protected $table = 'items';

    #[\Override]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

// A trait of the model itself does not count for #[\Override].
final class OwnTrait extends Model
{
    use HasFactory;

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

// An empty attribute group, which PHP fails to parse and Mago accepts.
final class EmptyGroup extends Model
{
    protected $table = 'items';

    #[]
    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

// An admitted getRouteKeyName leaves the output, and a call of it still stops.
final class Plain extends Model
{
    protected $table = 'items';

    public function getRouteKeyName(): string
    {
        return 'uuid';
    }
}

function keyOf(Plain $plain): string
{
    return $plain->getRouteKeyName();
}

// #[Override] without `use Override;` resolves to App\Override, another attribute, so
// P2R-401's rule applies.
final class Unresolved extends Model
{
    protected $table = 'items';

    #[Override]
    public function getRouteKeyName(): string
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

    // Condition 11 comes before condition 12.
    public function argumentFirst(): HasMany
    {
        return $this->hasMany(self::class)->latest();
    }

    // The other pivot links on another type, a constant in with(), and a dynamic name.
    public function pivotColumns(): HasMany
    {
        return $this->hasMany(Links::class)->withPivot('a');
    }

    public function pivotNull(): HasMany
    {
        return $this->hasMany(Links::class)->wherePivotNull('a');
    }

    public function withConstant(): HasMany
    {
        return $this->hasMany(Links::class)->with(PHP_EOL);
    }

    public function dynamic(): HasMany
    {
        return $this->hasMany(Links::class)->{'latest'}();
    }
}
