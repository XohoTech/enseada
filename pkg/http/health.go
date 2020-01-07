// Copyright 2019 Enseada authors
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

package http

import (
	"net/http"

	"github.com/enseadaio/enseada/internal/cachecontrol"

	"github.com/labstack/echo"
)

type HealthCheckResponse struct {
	Status   string `json:"status"`
	Protocol string `json:"protocol"`
	Host     string `json:"host"`
	Remote   string `json:"remote"`
	Method   string `json:"method"`
	Path     string `json:"path"`
}

func mountHealthCheck(e *echo.Echo) {
	e.GET("/health", func(c echo.Context) error {
		req := c.Request()
		res := HealthCheckResponse{
			Status:   "UP",
			Protocol: req.Proto,
			Host:     req.Host,
			Remote:   req.RemoteAddr,
			Method:   req.Method,
			Path:     req.URL.Path,
		}
		cc := cachecontrol.NoCache(true)
		cc.Write(c.Response().Writer)
		return c.JSON(http.StatusOK, res)
	})
}
