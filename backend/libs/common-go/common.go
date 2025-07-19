package common

import (
	"context"

	"github.com/gin-gonic/gin"
	"github.com/google/uuid"
	_ "github.com/jackc/pgx/v5/stdlib" // pgx driver for sqlx
	"github.com/jmoiron/sqlx"
	"github.com/joho/godotenv"
)

// UUIDGenerator generates UUIDs
type UUIDGenerator interface {
	NewID() string
}

// UUIDService implements UUIDGenerator
type UUIDService struct{}

func (u *UUIDService) NewID() string {
	return uuid.New().String()
}

// DBConn wraps basic DB operations
type DBConn interface {
	ExecContext(ctx context.Context, query string, args ...interface{}) error
	SelectContext(ctx context.Context, dest interface{}, query string, args ...interface{}) error
	GetContext(ctx context.Context, dest interface{}, query string, args ...interface{}) error
	Close() error
}

// SQLxDB implements DBConn using sqlx
type SQLxDB struct {
	db *sqlx.DB
}

func NewSQLxDB(dsn string) (*SQLxDB, error) {
	db, err := sqlx.Connect("pgx", dsn)
	if err != nil {
		return nil, err
	}
	return &SQLxDB{db: db}, nil
}

func (s *SQLxDB) ExecContext(ctx context.Context, query string, args ...interface{}) error {
	_, err := s.db.ExecContext(ctx, query, args...)
	return err
}

func (s *SQLxDB) SelectContext(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return s.db.SelectContext(ctx, dest, query, args...)
}

func (s *SQLxDB) GetContext(ctx context.Context, dest interface{}, query string, args ...interface{}) error {
	return s.db.GetContext(ctx, dest, query, args...)
}

func (s *SQLxDB) Close() error {
	return s.db.Close()
}

// Router wraps HTTP routes
type Router interface {
	GET(path string, handlers ...gin.HandlerFunc)
	POST(path string, handlers ...gin.HandlerFunc)
	Run(addr ...string) error
}

// GinRouter implements Router using gin
type GinRouter struct {
	engine *gin.Engine
}

func NewGinRouter() *GinRouter {
	return &GinRouter{engine: gin.Default()}
}

func (r *GinRouter) GET(path string, handlers ...gin.HandlerFunc) {
	r.engine.GET(path, handlers...)
}

func (r *GinRouter) POST(path string, handlers ...gin.HandlerFunc) {
	r.engine.POST(path, handlers...)
}

func (r *GinRouter) Run(addr ...string) error {
	return r.engine.Run(addr...)
}

// EnvLoader loads .env files
type EnvLoader interface {
	Load(filenames ...string) error
}

// DotenvLoader implements EnvLoader using godotenv
type DotenvLoader struct{}

func (d *DotenvLoader) Load(filenames ...string) error {
	return godotenv.Load(filenames...)
}
