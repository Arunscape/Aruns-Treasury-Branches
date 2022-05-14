package graph

// This file will be automatically regenerated based on the schema, any resolver implementations
// will be copied through when generating and any unknown code will be moved to the end.

import (
	"context"

	"github.com/Arunscape/Aruns-Treasury-Branches/atb-backend/graph/generated"
	"github.com/Arunscape/Aruns-Treasury-Branches/atb-backend/graph/model"
)

func (r *mutationResolver) SignUp(ctx context.Context, input model.NewUser) (*model.User, error) {

	user := &model.User{
		UUID:  input.UUID,
		Email: input.Email,
	}

	return user, nil
}

// Mutation returns generated.MutationResolver implementation.
func (r *Resolver) Mutation() generated.MutationResolver { return &mutationResolver{r} }

type mutationResolver struct{ *Resolver }
