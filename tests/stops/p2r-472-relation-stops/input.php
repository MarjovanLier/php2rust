<?php

declare(strict_types=1);

namespace App;

use Illuminate\Database\Eloquent\Model;
use Illuminate\Database\Eloquent\PendingHasThroughRelationship;
use Illuminate\Database\Eloquent\Relations\BelongsTo;
use Illuminate\Database\Eloquent\Relations\HasMany;
use Illuminate\Database\Eloquent\Relations\HasOne;
use Illuminate\Database\Eloquent\Relations\MorphToMany;
use Illuminate\Database\Eloquent\SoftDeletes;

final class Post extends Model
{
    protected $fillable = ['title'];

    public $timestamps = false;
}

// Relation methods that P2R-470 does not admit, each stopping with the first condition it
// fails, and two methods that keep P2R-401's message (P2R-472).
final class Author extends Model
{
    protected $fillable = ['name', 'title'];

    public $timestamps = false;

    // Names that PHP gives their own rules: it refuses both of these.
    public function __clone(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function __destruct(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    // Named like the column title in another letter case.
    public function Title(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    // The column comes before the modifier in P2R-470's order.
    protected function name(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function withLimit(int $limit): HasMany
    {
        return $this->hasMany(Post::class);
    }

    protected function drafts(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    final public function sealed(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    function bare(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    #[\Override]
    public function marked(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function &byReference(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function tags(): MorphToMany
    {
        return $this->morphToMany(Post::class, 'taggable');
    }

    public function pending(): PendingHasThroughRelationship
    {
        return $this->through('posts');
    }

    public function untyped()
    {
        return $this->hasMany(Post::class);
    }

    public function maybe(): ?HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function either(): HasMany|HasOne
    {
        return $this->hasMany(Post::class);
    }

    public function both(): HasMany&\Countable
    {
        return $this->hasMany(Post::class);
    }

    public function mismatched(): HasOne
    {
        return $this->hasMany(Post::class);
    }

    public function lowerHelper(): HasMany
    {
        return $this->hasmany(Post::class);
    }

    public function named(): HasMany
    {
        return $this->hasMany(related: Post::class);
    }

    public function unpacked(): HasMany
    {
        return $this->hasMany(...['App\Post']);
    }

    public function itself(): HasMany
    {
        return $this->hasMany(self::class);
    }

    public function computed(): HasMany
    {
        return $this->hasMany(strtolower(Post::class));
    }

    public function missing(): HasMany
    {
        return $this->hasMany(Missing::class);
    }

    public function twoSteps(): HasMany
    {
        $relation = $this->hasMany(Post::class);
        return $relation;
    }

    public function wrapped(): HasMany
    {
        return ($this->hasMany(Post::class));
    }

    // Methods that fail two conditions get the earlier one: a parameter (3) before the
    // modifier (4), the modifier (4) before the attribute (5), the attribute (5) before
    // the reference (6), and the reference (6) before a nullable type (7).
    protected function paramFirst(int $limit): HasMany
    {
        return $this->hasMany(Post::class);
    }

    #[\Override]
    protected function modifierFirst(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    #[\Override]
    public function &attributeFirst(): HasMany
    {
        return $this->hasMany(Post::class);
    }

    public function &referenceFirst(): ?HasMany
    {
        return $this->hasMany(Post::class);
    }

    // A relation type alone triggers P2R-472, and so does a plain call alone.
    public function nothing(): ?HasMany
    {
        return null;
    }

    public function formatted(): string
    {
        return $this->label();
    }

    public function viaStatic(): HasMany
    {
        return $this->hasMany(static::class);
    }

    public function viaParent(): HasMany
    {
        return $this->hasMany(parent::class);
    }

    public function viaConstant(): HasMany
    {
        return $this->hasMany(PHP_EOL);
    }

    public function label(): string
    {
        return 'author';
    }

    public function getRouteKeyName(): BelongsTo
    {
        return $this->belongsTo(Post::class);
    }
}

// An accessor shaped like a relation keeps P2R-401's rule, which takes the struct away.
final class Accessor extends Model
{
    public $timestamps = false;

    public function getTitleAttribute(): BelongsTo
    {
        return $this->belongsTo(Post::class);
    }
}

// An override of a method of a trait the model uses keeps P2R-401's rule too: Laravel
// calls getDeletedAtColumn on every model it builds.
final class Trashed extends Model
{
    use SoftDeletes;

    public $timestamps = false;

    public function getDeletedAtColumn(): HasMany
    {
        return $this->hasMany(Post::class);
    }
}
